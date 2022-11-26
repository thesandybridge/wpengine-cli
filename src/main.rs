mod api;
mod installs;
mod sites;
mod users;
mod ssh;
mod accounts;

fn main() {
    api::auth();
    users::handle_users();
    sites::get_sites();
    installs::handle_installs();
    ssh::handle_ssh();
    accounts::handle_accounts();
}
