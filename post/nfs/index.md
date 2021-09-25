
# nfs服务
## 安装
``` shell
sudo apt-get install nfs-kernel-server
```

## 设置导出
``` shell
/home/exports *(rw,nohide,insecure,no_subtree_check,async,no_root_squash)
```

## 开启服务
``` shell
sudo /etc/init.d/nfs-kernel-server restart
```

## 测试
``` shell
sudo mount -t nfs -o nolock,vers=3 127.0.0.1:/home/exports /mnt
ls /mnt
```