pub async fn make_deploy() {
    let deploy_command = std::env::var("DEPLOY_COMMAND").expect("DEPLOY_COMMAND must bet set.");
    let deploy_branch = std::env::var("DEPLOY_BRANCH").expect("DEPLOY_BRANCH must be set.");
    let deploy_message = std::env::var("DEPLOY_MESSAGE").expect("DEPLOY_MESSAGE must be set.");
    let deploy_dir = std::env::var("DEPLOY_DIR").expect("DEPLOY_DIR must be set.");
    let deploy_build_command =
        std::env::var("DEPLOY_BUILD_COMMAND").expect("DEPLOY_BUILD_COMMAND must be set.");
    let deploy_build_dir =
        std::env::var("DEPLOY_BUILDED_DIR").expect("DEPLOY_BUILDED_DIR must be set.");
}
