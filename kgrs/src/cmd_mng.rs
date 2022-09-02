//! There are command manager.

use colored::*;
use serenity::{
    builder::CreateApplicationCommand,
    http::client::Http,
    model::{application::command::Command, prelude::CommandId},
};

#[derive(Debug)]
#[non_exhaustive]
/// The command manager.
///
/// # Example
///
/// ```no_run
/// use kgrs::cmd_mng::{cmd_list, CmdManager};
/// use serenity::model::application::command::{
///     // May need these.
///     //CommandOptionType,
///     //CommandType,
/// };
///
/// # async run() -> {
/// // Prints command list.
/// // You can you this function to check which commands are there.
/// cmd_list(&ctx.http).await;
///
/// // Main manager.
/// // ! WARNING: If manage multiple commands at once, Clone the variable `cmd`.
/// // !          Recommend always cloning to avoid mistakes.
/// let cmd = serenity::builder::CreateApplicationCommand::default();
/// CmdManager::new()
/// 
///     // Creates command.
///     .create(cmd.clone()
///         .name("ping").description("pong!")
///         .description_localized("ja", "pong!")
///     })
/// 
///     // Edits command.
///     .edit(1014243185880465557, cmd.clone()
///         .name("info").description("Show bot information")
/// 
///     // Deletes command.
///     .delete(1014243185880465558)
/// 
///     .run(&ctx.http)
///     .await;
/// # }
/// ```
pub struct CmdManager {
    cmds_to_create: Vec<CreateApplicationCommand>,
    cmds_to_edit: Vec<(u64, CreateApplicationCommand)>,
    cmds_to_delete: Vec<u64>,
}

impl CmdManager {
    /// Creates a new command manager.
    pub fn new() -> Self {
        CmdManager {
            cmds_to_create: Vec::new(),
            cmds_to_edit: Vec::new(),
            cmds_to_delete: Vec::new(),
        }
    }

    /// Creates a new command.
    pub fn create(mut self, cmd: &CreateApplicationCommand) -> Self {
        self.cmds_to_create.push(cmd.to_owned());
        self
    }

    /// Edits a command.
    pub fn edit(mut self, id: u64, edited_cmd: &CreateApplicationCommand) -> Self {
        self.cmds_to_edit.push((id, edited_cmd.to_owned()));
        self
    }

    /// Deletes a command.
    pub fn delete(mut self, command_id: u64) -> Self {
        self.cmds_to_delete.push(command_id);
        self
    }

    /// Actually manages the commands
    pub async fn run(self, http: impl AsRef<Http>) {
        println!("[ == Command manager running... == ]");

        // Creates commands.
        for cmd in self.cmds_to_create {
            let created_cmd = Command::create_global_application_command(&http, |c| {
                *c = cmd;
                c
            })
            .await;
            match created_cmd {
                Ok(cmd) => println!("{}", format!("Command created: {}({})", cmd.name, cmd.id).green()),
                Err(why) => println!("{}", format!("Could not create a command: {}", why).red()),
            }
        }

        // Edits commands.
        for (id, cmd) in self.cmds_to_edit {
            let edited_cmd = Command::edit_global_application_command(&http, CommandId(id), |c| {
                *c = cmd;
                c
            })
            .await;
            match edited_cmd {
                Ok(cmd) => println!("{}", format!("Command edited: {}({})", cmd.name, cmd.id).green()),
                Err(why) => println!("{}", format!("Could not edit a command: {}", why).red()),
            }
        }

        // Deletes commands.
        for id in self.cmds_to_delete {
            let cmd = Command::delete_global_application_command(&http, CommandId(id)).await;
            match cmd {
                Ok(()) => println!("{}", format!("Command deleted: {}", id).green()),
                Err(why) => println!("{}", format!("Could not delete a command: {}", why).red()),
            }
        }

        println!("[ == Commands managed == ]");
    }
}

pub async fn cmd_list(http: impl AsRef<Http>) {
    let cmds = Command::get_global_application_commands(http)
        .await
        .expect("Could not get commands");
    println!("{}", format!("Current commands({}):", cmds.len()).cyan());
    let mut i = 1;
    for cmd in cmds {
        println!(
            "{}",
            format!(
                "{}. {}({}) | desc:{} | kind:{:?} |",
                i, cmd.name, cmd.id, cmd.description, cmd.kind
            )
            .cyan()
        );
        i += 1;
    }
}
