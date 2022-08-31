//! There are command manager.

use serenity::{
    model::{application::command::Command, prelude::CommandId},
    http::client::Http, 
    builder::CreateApplicationCommand
};
use colored::*;

#[derive(Debug)]
#[non_exhaustive]
/// The command manager.
pub struct CmdManager<Fc, Fe>
where
    Fc: FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
    Fe: FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
{
    cmds_to_create: Option<Vec<Fc>>,
    cmds_to_edit: Option<Vec<(u64, Fe)>>,
    cmds_to_delete: Option<Vec<u64>>,
}

impl<Fc, Fe> CmdManager<Fc, Fe>
where
    Fc: FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
    Fe: FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
{
    /// Creates a new command manager.
    pub fn new() -> Self {
        CmdManager {
            cmds_to_create: None,
            cmds_to_edit: None,
            cmds_to_delete: None,
        }
    }

    /// Creates a new command.
    pub fn create(self, f: Fc) -> Self {
        let cmds_to_create = Some(
            if let Some(mut v) = self.cmds_to_create {
                v.push(f);
                v
            } else {
                vec![f]
            }
        );
        Self {
            cmds_to_create,
            ..self
        }
    }

    /// Edits a command.
    pub fn edit(self, id: u64, f: Fe) -> Self {
        let t = (id, f);
        let cmds_to_edit = Some(
            if let Some(mut v) = self.cmds_to_edit {
                v.push(t);
                v
            } else {
                vec![t]
            }
        );
        Self {
            cmds_to_edit,
            ..self
        }
    }

    /// Deletes a command.
    pub fn delete(self, command_id: u64) -> Self {
        let cmds_to_delete = Some(
            if let Some(mut v) = self.cmds_to_delete {
                v.push(command_id);
                v
            } else {
                vec![command_id]
            }
        );
        Self {
            cmds_to_delete,
            ..self
        }
    }

    /// Actually manages the commands
    pub async fn run(self, http: impl AsRef<Http>) {
        println!("[ == Command manager running... == ]");

        // Creates commands.
        if let Some(ctc) = self.cmds_to_create {
            for f in ctc {
                let cmd = Command::create_global_application_command(&http, f).await;
                match cmd {
                    Ok(cmd) => println!("Command created: {}({})", cmd.name, cmd.id),
                    Err(why) => println!("Could not create a command: {}", why),
                }
            }
        }

        // Edits commands.
        if let Some(cte) = self.cmds_to_edit {
            for (id, f) in cte {
                let cmd = Command::edit_global_application_command(&http, CommandId(id), f).await;
                match cmd {
                    Ok(cmd) => println!("Command edited: {}({})", cmd.name, cmd.id),
                    Err(why) => println!("Could not edit a command: {}", why),
                }
            }
        }

        // Deletes commands.
        if let Some(ctd) = self.cmds_to_delete {
            for id in ctd {
                let cmd = Command::delete_global_application_command(&http, CommandId(id)).await;
                match cmd {
                    Ok(()) => println!("Command deleted: {}", id),
                    Err(why) => println!("Could not delete a command: {}", why),
                }
            }
        }

        println!("[ == Commands managed == ]");
    }
}

pub async fn cmd_list(http: impl AsRef<Http>) {
    let cmds = Command::get_global_application_commands(http).await.expect("Could not get commands");
    println!(
        "{}",
        format!("Current commands({}):", cmds.len()).cyan()
    );
    let mut i = 1;
    for cmd in cmds {
        println!(
            "{}",
            format!("{}. {}({}) | desc:{} | kind:{:?} |",
                i,
                cmd.name,
                cmd.id,
                cmd.description,
                cmd.kind
            ).cyan()
        );
        i +=1;
    }

}
