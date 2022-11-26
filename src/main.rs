mod api;
mod installs;
mod sites;
mod users;
mod ssh;
mod accounts;

fn main() {
    api::auth();
    users::handle_users();
    let sites = sites::get_sites();
    println!("{:#?}", sites);
    installs::handle_installs();
    ssh::handle_ssh();
    accounts::handle_accounts();
}
