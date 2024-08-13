## About opsx

```txt
This is a small git hook trigger based on rust axum, used for custom automation operations, custom packaging, uploading, and execution, with complete flexibility and autonomy.

It abandons the cumbersome and complex configuration process of tools like Jenkins.

I believe that the traditional combination of shell, expect, and scp is sufficient to complete many automated deployments.

Although this hook is still in its early stages, logs, proxies and different combinations will be added later.

The current version supports hooks from GitHub and GitLab.

And it can execute multiple tasks. The following is the structure setting:
{
        "name":"test",
        "security_key":"123456",
        "git_type":"gitlab",
        "git_url":"http://github.com",
        "git_branch":"test",
        "ext_script":"/opt/ext/xxx.sh",
}
name = The project name 'test' corresponds to the URL request name, such as the deployment address: xxxx.com/dev?keyword=test
security_key = GitHub or GitLab security code, used for hmac and other verifications. (PS: MD5 is recommended)
git_type = Hook type: GitHub, GitLab
git_url = git address
git_branch = The branch that triggers the deployment
ext_script = The execution script is based on asynchronous execution by tokio, hence it perfectly supports complex builds.
```


## Cfg examples of multiple hooks task

```json
[
    {
        "name":"test",
        "security_key":"123456",
        "git_type":"gitlab",
        "git_url":"http://github.com",
        "git_branch":"test",
        "ext_script":"/opt/ext/xxx.sh"
    },
    {
        "name":"test123",
        "security_key":"123456",
        "git_type":"gitlab",
        "git_url":"http://github.com",
        "git_branch":"test",
        "ext_script":"/opt/ext/xxx.sh"
    },
]

```

## cfg.json

```txt
    This is the configuration file, which is located in the root directory of the project.
```

## Development Environment

```txt
rust 1.82.0-nightly
axum 0.6.18
For other requirements, please refer to Cargo.toml
```

## Building

```txt
git clone git@github.com:x7c7v7i87/opsx.git

cargo build
```
