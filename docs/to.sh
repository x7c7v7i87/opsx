#!/bin/bash

# jq site docs
# https://jqlang.github.io/jq/
# JSON 文件 data.json：
# [
#     {
#         "ip": "127.0.0.1",
#         "name": "sg",
#         "key": "/opt/ssh/abc.pem",
#         "uname": "admin"
#     },
#     {
#         "name": "pt",
#         "ip": "127.0.0.2",
#         "key": "/opt/ssh/abc.pem",
#         "uname": "admin"
#     }
# ]

#this json file name 
JSON="/your path/data.json"

# 使用 jq 来查找匹配的对象
result_ser=$(jq -r . $JSON)

# 输出查找到的对象
echo "Server object:"
echo "$result_ser"

# 使用 read 函数读取用户的输入
echo "Enter a name to search for:"
read name

result=$(jq -c ".[] | select(.name == \"$name\")" $JSON)


ips=()
names=()
keys=()
unames=()


while IFS= read -r line; do

  ip=$(echo "$line" | jq -r '.ip')
  name=$(echo "$line" | jq -r '.name')
  key=$(echo "$line" | jq -r '.key')
  uname=$(echo "$line" | jq -r '.uname')
  ips+=("$ip")
  names+=("$name")
  keys+=("$key")
  unames+=("$uname")
done <<< "$result"


for i in "${!ips[@]}"; do
  ip="${ips[$i]}"
  name="${names[$i]}"
  key="${keys[$i]}"
  uname="${unames[$i]}"
  echo "start login server "
  echo "ssh -i $key $uname@$ip" 
  ssh -i $key $uname@$ip
done



