## 关于opsx

```text
这是一个基于rust axum 小型git钩子触发器，用于自定义自动化运维，自定义打包，上传，执行，完全灵活自主，抛弃Jenkins等笨重复杂的配置过程，我相信传统的shell，expect，scp这个组合足够完成很多自动化的部署，虽然这个钩子还是初期，后期会加入日志，代理以及不同的组合。

目前的版本支持github钩子
并且可以执行多任务，以下是结构的设定:
{
        "name":"test",
        "security_key":"123456",
        "git_type":"gitlab",
        "git_url":"http://github.com",
        "git_branch":"test",
        "ext_script":"/opt/ext/xxx.sh",
}
name=项目名称test 对应url 请求名称，如部署的地址为： xxxx.com/dev?keyword=test
security_key = github or gitlab 安全码，用于hmac等验证
git_type = 钩子类型，目前只支持github
git_url = git地址
git_branch = 触发部署的分支
ext_script = 执行脚本是基于tokio异步执行的，所以复杂的构建很完美支持..
```

#多个钩子例子
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

```text
这是配置文件，该配置存在于项目根目录
```

## 开发版本环境

```text
rust 1.68
axum 0.6.18
其他请参考
Cargo.toml
```

## 构建
```text
git clone git@github.com:x7c7v7i87/opsx.git

cargo build

````

