use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use ppdrive::state::AppState;
use ppdrive::tools::system_info::{SystemInfo, NetworkThroughput};
use crate::data::{Claims, OverviewResponse};

pub async fn overview_handler(
    State(state): State<AppState>,
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
) -> Result<Json<OverviewResponse>, (StatusCode, Json<crate::ErrorResponse>)> {
    let db = state.db();
    let server_name = &state.config().server_name;

    let total_users = ppdrive::db::stats::count_users(db).await
        .map_err(|e| crate::err_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let since = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() - 7 * 86400) as i64;
    let since_rfc3339 = chrono::DateTime::from_timestamp(since, 0)
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default();
    let added_this_week = ppdrive::db::stats::count_users_after(db, &since_rfc3339).await
        .map_err(|e| crate::err_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let total_clients = ppdrive::db::stats::count_clients(db).await
        .map_err(|e| crate::err_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let total_assets = ppdrive::db::stats::count_assets(db).await
        .map_err(|e| crate::err_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let total_buckets = ppdrive::db::stats::count_buckets(db).await
        .map_err(|e| crate::err_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let host = SystemInfo::gather(server_name);

    let root = state.config().root_dir()
        .map_err(|e| crate::err_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;
    let storage_used = SystemInfo::storage_used(&root);

    let throughput: NetworkThroughput = tokio::task::spawn_blocking(NetworkThroughput::sample)
        .await
        .map_err(|e| crate::err_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let response = OverviewResponse {
        logged_user: claims.sub,
        storage_used: format_bytes(storage_used),
        counter: crate::data::Counter {
            users: crate::data::UserCounter {
                total: total_users as usize,
                added_this_week: added_this_week as usize,
            },
            clients: crate::data::ClientCounter {
                total: total_clients as usize,
                active: total_clients as usize,
            },
            objects: crate::data::ObjectCounter {
                total: total_assets as usize,
                buckets: total_buckets as usize,
            },
        },
        host: crate::data::HostInfo {
            os: host.os,
            kernel: host.kernel,
            architecture: host.architecture,
            hostname: host.hostname,
            uptime: host.uptime,
            cpu: host.cpu,
            cpu_cores: host.cpu_cores,
            load_average: host.load_average,
            used_storage: host.used_storage,
            total_storage: host.total_storage,
        },
        throughput: crate::data::Throughput {
            ingres: throughput.ingres,
            egres: throughput.egres,
            req_per_second: 0,
            latency_p99: "N/A".to_string(),
        },
    };

    Ok(Json(response))
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes < 1024u64 * 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else {
        format!("{:.2} TB", bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0))
    }
}
