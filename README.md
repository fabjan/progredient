# progredient

Just a program wrapping the nice and simple https://github.com/dominicparga/progressing for direct use in scripts etc.

## Example usage

```shell
df | grep disk | while read label blocks used available rest; do progredient --from 0 --to $available --at $used --label $label --style '[|| ]' ; done
[||||||            ] /dev/disk1s5s1
[||                ] /dev/disk1s4
[|                 ] /dev/disk1s2
[|                 ] /dev/disk1s6
[||||||||||||||||||] /dev/disk1s1
[||||||            ] /dev/disk1s5
```
