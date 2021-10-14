use color_eyre::eyre::{Context, Result};
use http::StatusCode;
use ppm_prototype::{
    aggregate::VerifyStartRequest,
    collect::OutputShareRequest,
    helper::Helper,
    hpke::{self, Role},
    parameters::Parameters,
    trace, with_shared_value,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::{error, info};
use warp::{reply, Filter};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    trace::install_subscriber();

    let ppm_parameters = Parameters::from_config_file().wrap_err("loading task parameters")?;
    let port = ppm_parameters.aggregator_urls[Role::Helper.index()]
        .port()
        .unwrap_or(80);

    let hpke_config =
        hpke::Config::from_config_file(Role::Helper).wrap_err("loading HPKE config")?;
    let hpke_config_endpoint = hpke_config.warp_endpoint();

    let aggregate = warp::post()
        .and(warp::path("aggregate"))
        .and(warp::body::json())
        .and(with_shared_value(ppm_parameters.clone()))
        .and(with_shared_value(hpke_config.clone()))
        .map(
            move |aggregate_request: VerifyStartRequest,
                  ppm_parameters: Parameters,
                  hpke_config: hpke::Config| {
                // We intentionally create a new instance of Helper every time we
                // handle a request to prove that we can successfully execute the
                // protocol without maintaining local state
                let mut helper_aggregator = match Helper::new(
                    &ppm_parameters,
                    &hpke_config,
                    &aggregate_request.helper_state,
                ) {
                    Ok(helper) => helper,
                    Err(e) => {
                        error!(error = ?e, "failed to create helper aggregator with state");
                        return reply::with_status(reply::json(&()), StatusCode::BAD_REQUEST);
                    }
                };

                match helper_aggregator.handle_aggregate(&aggregate_request) {
                    Ok(response) => reply::with_status(reply::json(&response), StatusCode::OK),
                    Err(e) => {
                        error!(error = ?e, "failed to handle aggregate request");
                        reply::with_status(reply::json(&()), StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            },
        )
        .with(warp::trace::named("aggregate"));

    let output_share = warp::post()
        .and(warp::path("output_share"))
        .and(warp::body::json())
        .and(with_shared_value(ppm_parameters.clone()))
        .and(with_shared_value(hpke_config.clone()))
        .map(
            move |output_share_request: OutputShareRequest,
                  ppm_parameters: Parameters,
                  hpke_config: hpke::Config| {
                let mut helper_aggregator = match Helper::new(
                    &ppm_parameters,
                    &hpke_config,
                    &output_share_request.helper_state,
                ) {
                    Ok(helper) => helper,
                    Err(_) => {
                        return reply::with_status(reply::json(&()), StatusCode::BAD_REQUEST);
                    }
                };

                match helper_aggregator.handle_output_share(&output_share_request) {
                    Ok(response) => reply::with_status(reply::json(&response), StatusCode::OK),
                    Err(_) => {
                        reply::with_status(reply::json(&()), StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            },
        )
        .with(warp::trace::named("output_share"));

    let routes = hpke_config_endpoint
        .or(aggregate)
        .or(output_share)
        .with(warp::trace::request());

    info!("helper serving on 0.0.0.0:{}", port);
    warp::serve(routes)
        .run(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))
        .await;

    unreachable!()
}
