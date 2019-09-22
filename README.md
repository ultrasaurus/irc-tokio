
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




## Understanding IRC protocol

The folllowing was captured with an IRC client

```
> 2019/09/21 14:12:28.166479  length=123 from=0 to=122
PASS [redacted]\r
NICK ultrasaurus_twitter\r
USER ultrasaurus_twitter 0 * ultrasaurus_twitter\r
< 2019/09/21 14:12:28.270430  length=82 from=0 to=81
:ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im NICK :ultrasaurus_twitter\r
< 2019/09/21 14:12:28.271433  length=867 from=82 to=948
:irc.gitter.im 001 ultrasaurus_twitter Gitter :ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im\r
:irc.gitter.im 002 ultrasaurus_twitter Version: 1.4.0\r
:irc.gitter.im 003 ultrasaurus_twitter Created on Wed Sep 18 2019 22:01:04 GMT+0000 (Coordinated Universal Time)\r
:irc.gitter.im 004 ultrasaurus_twitter irc.gitter.im 1.4.0 wo ntr\r
:irc.gitter.im 005 ultrasaurus_twitter CHANTYPES=# CHANMODES=,,, CHANNELLEN=128 NICKLEN=40 NETWORK=Gitter SAFELIST CASEMAPPING=ascii :are supported by this server\r
:irc.gitter.im 375 ultrasaurus_twitter :- Message of the Day -\r
:irc.gitter.im 372 ultrasaurus_twitter :- Welcome To Gitter IRC!\r
:irc.gitter.im 372 ultrasaurus_twitter :- Info at https://irc.gitter.im\r
:irc.gitter.im 372 ultrasaurus_twitter :- Code at https://gitlab.com/gitlab-org/gitter/irc-bridge\r
:irc.gitter.im 376 ultrasaurus_twitter :End of /MOTD command.\r
> 2019/09/21 14:12:28.282415  length=19 from=123 to=141
JOIN #ultrasaurus\r
< 2019/09/21 14:12:30.821076  length=77 from=949 to=1025
:ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im JOIN #tokio-rs/tokio\r
< 2019/09/21 14:12:30.821933  length=438 from=1026 to=1463
:irc.gitter.im 332 ultrasaurus_twitter #tokio-rs/tokio :\r
:irc.gitter.im 353 ultrasaurus_twitter * #tokio-rs/tokio :LinuxMercedes zklapow baloo arcrose hnakamur rkusa bheesham markuskobler badboy tiziano88 Licenser jeromegn alex scrogson Hoverbear jxson piscisaureus kphelps porcow HelveticaScenario cmr 8573 typester tomgco reset mloc etehtsea pimeys debris astro gitter\r
:irc.gitter.im 366 ultrasaurus_twitter #tokio-rs/tokio : /NAMES\r
< 2019/09/21 14:12:30.827363  length=75 from=1464 to=1538
:ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im JOIN #tokio-rs/dev\r
< 2019/09/21 14:12:30.827981  length=447 from=1539 to=1985
:irc.gitter.im 332 ultrasaurus_twitter #tokio-rs/dev :\r
:irc.gitter.im 353 ultrasaurus_twitter * #tokio-rs/dev :baloo piscisaureus kphelps mies srijs zkat fundon Aaron1011 lukesteensen davidbarsky mitsuhiko chandu0101 jxs deamme xonatius jokeyrhyme LucioFranco kristoferB anp AGausmann vladimir-lu bryanburgers djc rubdos KevinMGranger bIgBV alevy rylev seanmonstar halfhorst gitter\r
:irc.gitter.im 366 ultrasaurus_twitter #tokio-rs/dev : /NAMES\r
> 2019/09/21 14:12:30.857923  length=22 from=142 to=163
MODE #tokio-rs/tokio\r
> 2019/09/21 14:12:30.864537  length=20 from=164 to=183
MODE #tokio-rs/dev\r
> 2019/09/21 14:12:39.883001  length=25 from=184 to=208
PRIVMSG #ultrasaurus hi\r
< 2019/09/21 14:12:58.167738  length=21 from=1986 to=2006
PING :irc.gitter.im\r
> 2019/09/21 14:12:58.169469  length=20 from=209 to=228
PONG irc.gitter.im\r
```