mod auth;
mod installs;
mod sites;
mod users;
mod ssh;
mod accounts;

fn main() {
    auth::handle_auth();
    users::handle_users();
    sites::handle_sites();
    installs::handle_installs();
    ssh::handle_ssh();
    accounts::handle_accounts();
}
