use rocket::fairing::AdHoc;
use utoipa::OpenApi;
pub mod proposal;
pub mod rfp;
pub mod sputnik;
use crate::db::db_types::ProposalWithLatestSnapshotView;
use crate::types::PaginatedResponse;
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Devhub Cache API",
        version = "0.0.1",
    ),
    paths(
      proposal::get_proposals,
      proposal::get_proposal
    ),
    components(schemas(
      PaginatedResponse<ProposalWithLatestSnapshotView>,
      // Json<VersionedProposal>
    )),
    tags(
        (name = "Devhub Cache", description = "Devhub cache endpoints.")
    ),
)]
pub struct ApiDoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Installing entrypoints", |rocket| async {
        rocket
            .attach(proposal::stage())
            .attach(rfp::stage())
            .attach(sputnik::stage())
    })
}
