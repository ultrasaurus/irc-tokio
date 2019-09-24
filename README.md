
## Getting started with IRC

1. join gitter
2. get IRC password from: https://irc.gitter.im/
3. on the command-line

```
socat -v tcp4-listen:1234,reuseaddr,fork ssl:irc.gitter.im:6667,verify=0
```

4. in another terminal window, in local repo directory

```cargo run```

socat output:
```
> 2019/09/21 17:07:51.380401  length=123 from=0 to=122
PASS 20dfff9c75f6b2a239f86223fea60c652d20f824\r
NICK ultrasaurus_twitter\r
USER ultrasaurus_twitter 0 * ultrasaurus_twitter\r
< 2019/09/21 17:07:51.476298  length=82 from=0 to=81
:ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im NICK :ultrasaurus_twitter\r

```




