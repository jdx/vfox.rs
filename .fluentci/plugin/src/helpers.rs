use extism_pdk::FnResult;
use fluentci_pdk::dag;

pub fn set_envs() -> FnResult<()> {
    let path = dag().get_env("PATH")?;
    let home = dag().get_env("HOME")?;
    let path = format!("{}/.cargo/bin:{}", home, path);
    dag().set_envs(vec![("PATH".into(), path)])?;
    Ok(())
}
