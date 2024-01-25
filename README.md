# moar-threads

```
$ cat /etc/sysctl.d/moar-threads.conf
kernel.threads-max=4000000
vm.max_map_count=9000000
$ grep NPROC /etc/systemd/system.conf
DefaultLimitNPROC=300000:300000
```
