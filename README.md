# HEU抢课脚本 in Rust

## TODO:

- [x] 配置文件读取个人信息（账号，密码，课程编号等等）
- [x] 优化 log，记录下已经抢到的课程以及对应账号
- [ ] 增加多用户功能，通过一个配置文件，完成多个账号的抢课任务
- [ ] 优化课程的筛选，增加fuzzy匹配

## 配置文件:

在源文件/可执行文件所在的目录或其父目录创建 `config.yaml`，格式如下：

`class` 中填写脚本初始化阶段输出的课程对应编号。
```yaml
account: 114514
password: 1919810
class: [1, 1, 4, 5, 1, 4]
```
