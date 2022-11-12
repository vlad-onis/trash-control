use crate::trash::trash_control::TrashControl;

mod trash;

fn main() {
    let args = trash::trash_cli::read_cli_arg();

    let tc = TrashControl {};

    tc.run(args);
}
