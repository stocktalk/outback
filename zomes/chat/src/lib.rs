//Much of this is based on the work here:
//https://github.com/holochain/holochain/tree/develop/crates/test_utils/wasm/wasm_workspace/whoami
// whih follows this license: https://github.com/holochain/holochain/blob/develop/LICENSE
use hdk::prelude::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub chat: String,
    pub content: String,
    pub uid: String,
    pub attachments: Option<Vec<String>>,
    pub encrypt_algo: Option<String>
}

impl Default for Message {
    fn default() -> Self {
        Message {
            id: Uuid::new_v4().to_string(),
            chat: Uuid::new_v4().to_string(),
            content: String::default().to_string(),
            uid: Uuid::new_v4().to_string(),
            attachments: None,
            encrypt_algo: None,
        }
    }
}


#[hdk_extern]
fn set_access(_: ()) -> ExternResult<()> {
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info()?.zome_name, "chat".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(())
}

// returns recent messages
#[hdk_extern]
fn recent_messages(_: ()) -> ExternResult<Vec<Message>> {
    Ok(vec![Message::default()])
}

//what messages
#[hdk_extern]
fn what_messages(agent_pubkey: AgentPubKey) -> ExternResult<Vec<Message>> {
    let zome_call_response: ZomeCallResponse = call_remote(
        agent_pubkey,
        zome_info()?.zome_name,
        "chat".to_string().into(),
        None,
        &(),
    )?;
    match zome_call_response {
        // The decode() type needs to match the return type of recent_messages
        ZomeCallResponse::Ok(v) => Ok(v.decode()?),
        // This should be handled in real code.
        _ => unreachable!(),
    }
}

// returns the messages reported by the given pub key
#[hdk_extern]
fn what_messages_local(cell_id: CellId) -> ExternResult<Vec<Message>> {
    let zome_call_response: ZomeCallResponse = call(
        Some(cell_id),
        zome_info()?.zome_name,
        "chat".to_string().into(),
        None,
        &(),
    )?;
    match zome_call_response {
        ZomeCallResponse::Ok(v) => Ok(v.decode()?),
        // This should be handled in real code.
        _ => unreachable!(),
    }
}

/// Call the create entry zome from this zome.
/// The cell id must point to a cell which includes
/// the "create_entry" zome.
#[hdk_extern]
fn call_create_entry(cell_id: CellId) -> ExternResult<HeaderHash> {
    let zome_call_response: ZomeCallResponse = call(
        Some(cell_id),
        "create_entry".to_string().into(),
        "create_entry".to_string().into(),
        None,
        &(),
    )?;
    match zome_call_response {
        ZomeCallResponse::Ok(v) => Ok(v.decode()?),
        // This should be handled in real code.
        _ => unreachable!(),
    }
}
