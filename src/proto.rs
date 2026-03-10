use crate::config::ClaudeCodePluginConfig;
use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

static NAME: &str = "Claude Code";

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::CommandLine,
        plugin_version: Some(env!("CARGO_PKG_VERSION").parse().unwrap()),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/anthropics/claude-code")?;

    let tags = tags
        .iter()
        .filter_map(|tag| tag.strip_prefix('v'))
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations![
            HostOS::MacOS => [HostArch::Arm64, HostArch::X64],
            HostOS::Linux => [HostArch::Arm64, HostArch::X64],
            HostOS::Windows => [HostArch::X64, HostArch::Arm64],
        ],
    )?;

    let version = input.context.version.to_string();
    let config = get_tool_config::<ClaudeCodePluginConfig>()?;
    let platform = get_claude_platform(&env)?;

    let binary_name = if env.os == HostOS::Windows {
        "claude.exe"
    } else {
        "claude"
    };

    let download_url = format!(
        "{}/{}/{}/{}",
        config.dist_url, version, platform, binary_name
    );

    Ok(Json(DownloadPrebuiltOutput {
        download_url,
        download_name: Some(binary_name.into()),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    let exe_name = if env.os == HostOS::Windows {
        "claude.exe"
    } else {
        "claude"
    };

    let mut exes = HashMap::default();
    exes.insert("claude".to_string(), ExecutableConfig::new(exe_name));

    Ok(Json(LocateExecutablesOutput {
        exes,
        ..LocateExecutablesOutput::default()
    }))
}

fn get_claude_platform(env: &HostEnvironment) -> FnResult<String> {
    let os = match env.os {
        HostOS::MacOS => "darwin",
        HostOS::Linux => "linux",
        HostOS::Windows => "win32",
        _ => {
            return Err(plugin_err!(PluginError::UnsupportedOS {
                tool: NAME.into(),
                os: env.os.to_string(),
            }));
        }
    };

    let arch = match env.arch {
        HostArch::Arm64 => "arm64",
        HostArch::X64 => "x64",
        _ => {
            return Err(plugin_err!(PluginError::UnsupportedArch {
                tool: NAME.into(),
                arch: env.arch.to_string(),
            }));
        }
    };

    let libc_suffix = if env.os == HostOS::Linux {
        match env.libc {
            HostLibc::Musl => "-musl",
            _ => "",
        }
    } else {
        ""
    };

    Ok(format!("{}-{}{}", os, arch, libc_suffix))
}
