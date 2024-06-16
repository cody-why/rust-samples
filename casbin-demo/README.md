# Casbin Demo
casbin的测试,在线生成规则:
 * https://casbin.org/zh/editor/
 * https://github.com/casbin/casbin-rs/tree/master/examples

### sqlx 宏错误消除
设置正确的数据库连接信息:
export DATABASE_URL=mysql://root:789789@192.168.1.199:3306/casbin
cargo build


## casbin模型
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && (p.obj == '*' && KeyMatch(r.obj, p.obj)) && regexMatch(r.act, p.act)


# 用户角色
g(user1, admin)
g(user2, user)

# 菜单权限
p(admin, *, " ")
p(user, /notice/todo, GET)
p(user, /notice/notice, GET)


# 单独设置某个用户权限
p(user1, /system/user, read)

# casbin 验证



