use ko_protocol::ckb_sdk::constants::TYPE_ID_CODE_HASH;
use ko_protocol::ckb_sdk::rpc::ckb_indexer::{IndexerRpcClient, Order, ScriptType, SearchKey};
use ko_protocol::ckb_sdk::traits::LiveCell;
use ko_protocol::ckb_types::packed::{CellOutput, Script};
use ko_protocol::ckb_types::prelude::{Builder, Entity, Pack};
use ko_protocol::ckb_types::{bytes::Bytes, core::ScriptHashType};
use ko_protocol::types::error::KoError;
use ko_protocol::{KoResult, is_mol_flag_2, mol_flag_0, mol_flag_1, mol_deployment_raw};

use crate::error::AssemblerError;

pub fn search_project_cell(
    rpc: &mut IndexerRpcClient,
    project_id_args: &[u8; 32]
) -> KoResult<LiveCell> {
    let project_typescript = Script::new_builder()
        .code_hash(TYPE_ID_CODE_HASH.pack())
        .hash_type(ScriptHashType::Type.into())
        .args(Bytes::from(project_id_args.to_vec()).pack())
        .build();
    let search_key = SearchKey {
        script: project_typescript.into(),
        script_type: ScriptType::Type,
        filter: None,
    };
    let result = rpc
        .get_cells(search_key, Order::Asc, 1.into(), None)
        .map_err(|_| KoError::from(AssemblerError::MissProjectDeploymentCell(project_id_args.clone())))?;
    if let Some(cell) = result.objects.first() {
        Ok((cell.clone()).into())
    } else {
        Err(KoError::from(AssemblerError::MissProjectDeploymentCell(
            project_id_args.clone(),
        )))
    }
}

pub fn search_global_cell(
    rpc: &mut IndexerRpcClient,
    code_hash: &[u8; 32],
    project_id: &[u8; 32],
) -> KoResult<LiveCell> {
    let global_typescript = Script::new_builder()
        .code_hash(code_hash.pack())
        .hash_type(ScriptHashType::Data1.into())
        .args(Bytes::from(mol_flag_0(project_id)).pack())
        .build();
    let search_key = SearchKey {
        script: global_typescript.into(),
        script_type: ScriptType::Lock,
        filter: None,
    };
    let result = rpc
        .get_cells(search_key, Order::Asc, 1.into(), None)
        .map_err(|_| KoError::from(AssemblerError::MissProjectGlobalCell(project_id.clone())))?;
    if let Some(cell) = result.objects.first() {
        Ok((cell.clone()).into())
    } else {
        Err(KoError::from(AssemblerError::MissProjectGlobalCell(
            project_id.clone(),
        )))
    }
}

pub fn make_global_script(code_hash: &[u8; 32], project_id: &[u8; 32]) -> Script {
    Script::new_builder()
        .code_hash(code_hash.pack())
        .hash_type(ScriptHashType::Data1.into())
        .args(Bytes::from(mol_flag_0(project_id)).pack())
        .build()
}

pub fn make_personal_script(code_hash: &[u8; 32], project_id: &[u8; 32]) -> Script {
    Script::new_builder()
        .code_hash(code_hash.pack())
        .hash_type(ScriptHashType::Data1.into())
        .args(Bytes::from(mol_flag_1(project_id)).pack())
        .build()
}

pub fn check_valid_request(cell: &CellOutput, code_hash: &[u8; 32], project_id: &[u8; 32]) -> bool {
    let lock = cell.lock();
    if lock.code_hash().as_slice() != &code_hash[..]
        || lock.hash_type() != ScriptHashType::Data1.into()
        || !is_mol_flag_2(&lock.args().raw_data().to_vec())
    {
        return false;
    }
    if let Some(type_) = cell.type_().to_opt() {
        if type_.as_slice() != make_personal_script(code_hash, project_id).as_slice() {
            return false;
        }
    } else {
        return false;
    }
    return true;
}

pub fn extract_project_lua_code(deployment_bytes: &Bytes) -> KoResult<Bytes> {
    if let Some(deployment) = mol_deployment_raw(&deployment_bytes) {
        Ok(deployment.code().raw_data())
    } else {
        Err(AssemblerError::UnsupportedDeploymentFormat.into())
    }
}
