Options:

  -?
       Print this help message and exit

  -alertnotify=<cmd>
       Execute command when an alert is raised (%s in cmd 
is replaced by
       message)

  -allowignoredconf
       For backwards compatibility, treat an unused bitcoi
n.conf file in the
       datadir as a warning, not an error.

  -assumevalid=<hex>
       If this block is in the chain assume that it and it
s ancestors are valid
       and potentially skip their script verification (0 t
o verify all,
       default:
       000000000000000000011c5890365bdbe5d25b97ce0057589ac
aef4f1a57263f,
       testnet3:
       000000000000000465b1a66c9f386308e8c75acef9201f3f577
811da09fc90ad,
       testnet4:
       000000005be348057db991fa5d89fe7c4695b667cfb311391a8
db374b6f681fd,
       signet:
       0000014aad1d58dddcb964dd749b073374c6306e716b22f573a
2efe68d414539)

  -blockfilterindex=<type>
       Maintain an index of compact filters by block (defa
ult: 0, values:
       basic). If <type> is not supplied or if <type> = 1,
 indexes for
       all known types are enabled.

  -blocknotify=<cmd>
       Execute command when the best block changes (%s in 
cmd is replaced by
       block hash)

  -blockreconstructionextratxn=<n>
       Extra transactions to keep in memory for compact bl
ock reconstructions
       (default: 100)

  -blocksdir=<dir>
       Specify directory to hold blocks subdirectory for *
.dat files (default:
       <datadir>)

  -blocksonly
       Whether to reject transactions from network peers. 
Disables automatic
       broadcast and rebroadcast of transactions, unless t
he source peer
       has the 'forcerelay' permission. RPC transactions a
re not
       affected. (default: 0)

  -blocksxor
       Whether an XOR-key applies to blocksdir *.dat files
. The created XOR-key
       will be zeros for an existing blocksdir or when `-b
locksxor=0` is
       set, and random for a freshly initialized blocksdir
. (default: 1)

  -coinstatsindex
       Maintain coinstats index used by the gettxoutsetinf
o RPC (default: 0)

  -conf=<file>
       Specify path to read-only configuration file. Relat
ive paths will be
       prefixed by datadir location (only useable from com
mand line, not
       configuration file) (default: bitcoin.conf)

  -daemon
       Run in the background as a daemon and accept comman
ds (default: 0)

  -daemonwait
       Wait for initialization to be finished before exiti
ng. This implies
       -daemon (default: 0)

  -datadir=<dir>
       Specify data directory

  -dbcache=<n>
       Maximum database cache size <n> MiB (4 to 16384, de
fault: 450). In
       addition, unused mempool memory is shared for this 
cache (see
       -maxmempool).

  -debuglogfile=<file>
       Specify location of debug log file (default: debug.
log). Relative paths
       will be prefixed by a net-specific datadir location
. Pass
       -nodebuglogfile to disable writing the log to a fil
e.

  -includeconf=<file>
       Specify additional configuration file, relative to 
the -datadir path
       (only useable from configuration file, not command 
line)

  -loadblock=<file>
       Imports blocks from external file on startup

  -maxmempool=<n>
       Keep the transaction memory pool below <n> megabyte
s (default: 300)

  -maxorphantx=<n>
       Keep at most <n> unconnectable transactions in memo
ry (default: 100)

  -mempoolexpiry=<n>
       Do not keep transactions in the mempool longer than
 <n> hours (default:
       336)

  -par=<n>
       Set the number of script verification threads (0 = 
auto, up to 15, <0 =
       leave that many cores free, default: 0)

  -persistmempool
       Whether to save the mempool on shutdown and load on
 restart (default: 1)

  -persistmempoolv1
       Whether a mempool.dat file created by -persistmempo
ol or the savemempool
       RPC will be written in the legacy format (version 1
) or the
       current format (version 2). This temporary option w
ill be removed
       in the future. (default: 0)

  -pid=<file>
       Specify pid file. Relative paths will be prefixed b
y a net-specific
       datadir location. (default: bitcoind.pid)

  -prune=<n>
       Reduce storage requirements by enabling pruning (de
leting) of old
       blocks. This allows the pruneblockchain RPC to be c
alled to
       delete specific blocks and enables automatic prunin
g of old
       blocks if a target size in MiB is provided. This mo
de is
       incompatible with -txindex. Warning: Reverting this
 setting
       requires re-downloading the entire blockchain. (def
ault: 0 =
       disable pruning blocks, 1 = allow manual pruning vi
a RPC, >=550 =
       automatically prune block files to stay under the s
pecified
       target size in MiB)

  -reindex
       If enabled, wipe chain state and block index, and r
ebuild them from
       blk*.dat files on disk. Also wipe and rebuild other
 optional
       indexes that are active. If an assumeutxo snapshot 
was loaded,
       its chainstate will be wiped as well. The snapshot 
can then be
       reloaded via RPC.

  -reindex-chainstate
       If enabled, wipe chain state, and rebuild it from b
lk*.dat files on
       disk. If an assumeutxo snapshot was loaded, its cha
instate will
       be wiped as well. The snapshot can then be reloaded
 via RPC.

  -settings=<file>
       Specify path to dynamic settings data file. Can be 
disabled with
       -nosettings. File is written at runtime and not mea
nt to be
       edited by users (use bitcoin.conf instead for custo
m settings).
       Relative paths will be prefixed by datadir location
. (default:
       settings.json)

  -shutdownnotify=<cmd>
       Execute command immediately before beginning shutdo
wn. The need for
       shutdown may be urgent, so be careful not to delay 
it long (if
       the command doesn't require interaction with the se
rver, consider
       having it fork into the background).

  -startupnotify=<cmd>
       Execute command on startup.

  -txindex
       Maintain a full transaction index, used by the getr
awtransaction rpc
       call (default: 0)

  -version
       Print version and exit

Connection options:

  -addnode=<ip>
       Add a node to connect to and attempt to keep the co
nnection open (see
       the addnode RPC help for more info). This option ca
n be specified
       multiple times to add multiple nodes; connections a
re limited to
       8 at a time and are counted separately from the -ma
xconnections
       limit.

  -asmap=<file>
       Specify asn mapping used for bucketing of the peers
 (default:
       ip_asn.map). Relative paths will be prefixed by the
 net-specific
       datadir location.

  -bantime=<n>
       Default duration (in seconds) of manually configure
d bans (default:
       86400)

  -bind=<addr>[:<port>][=onion]
       Bind to given address and always listen on it (defa
ult: 0.0.0.0). Use
       [host]:port notation for IPv6. Append =onion to tag
 any incoming
       connections to that address and port as incoming To
r connections
       (default: 127.0.0.1:8334=onion, testnet3: 127.0.0.1
:18334=onion,
       testnet4: 127.0.0.1:48334=onion, signet: 127.0.0.1:
38334=onion,
       regtest: 127.0.0.1:18445=onion)

  -cjdnsreachable
       If set, then this host is configured for CJDNS (con
necting to fc00::/8
       addresses would lead us to the CJDNS network, see d
oc/cjdns.md)
       (default: 0)

  -connect=<ip>
       Connect only to the specified node; -noconnect disa
bles automatic
       connections (the rules for this peer are the same a
s for
       -addnode). This option can be specified multiple ti
mes to connect
       to multiple nodes.

  -discover
       Discover own IP addresses (default: 1 when listenin
g and no -externalip
       or -proxy)

  -dns
       Allow DNS lookups for -addnode, -seednode and -conn
ect (default: 1)

  -dnsseed
       Query for peer addresses via DNS lookup, if low on 
addresses (default: 1
       unless -connect used or -maxconnections=0)

  -externalip=<ip>
       Specify your own public address

  -fixedseeds
       Allow fixed seeds if DNS seeds don't provide peers 
(default: 1)

  -forcednsseed
       Always query for peer addresses via DNS lookup (def
ault: 0)

  -i2pacceptincoming
       Whether to accept inbound I2P connections (default:
 1). Ignored if
       -i2psam is not set. Listening for inbound I2P conne
ctions is done
       through the SAM proxy, not by binding to a local ad
dress and
       port.

  -i2psam=<ip:port>
       I2P SAM proxy to reach I2P peers and accept I2P con
nections (default:
       none)

  -listen
       Accept connections from outside (default: 1 if no -
proxy, -connect or
       -maxconnections=0)

  -listenonion
       Automatically create Tor onion service (default: 1)

  -maxconnections=<n>
       Maintain at most <n> automatic connections to peers
 (default: 125). This
       limit does not apply to connections manually added 
via -addnode
       or the addnode RPC, which have a separate limit of 
8.

  -maxreceivebuffer=<n>
       Maximum per-connection receive buffer, <n>*1000 byt
es (default: 5000)

  -maxsendbuffer=<n>
       Maximum per-connection memory usage for the send bu
ffer, <n>*1000 bytes
       (default: 1000)

  -maxuploadtarget=<n>
       Tries to keep outbound traffic under the given targ
et per 24h. Limit
       does not apply to peers with 'download' permission 
or blocks
       created within past week. 0 = no limit (default: 0M
). Optional
       suffix units [k|K|m|M|g|G|t|T] (default: M). Lowerc
ase is 1000
       base while uppercase is 1024 base

  -networkactive
       Enable all P2P network activity (default: 1). Can b
e changed by the
       setnetworkactive RPC command

  -onion=<ip:port|path>
       Use separate SOCKS5 proxy to reach peers via Tor on
ion services, set
       -noonion to disable (default: -proxy). May be a loc
al file path
       prefixed with 'unix:'.

  -onlynet=<net>
       Make automatic outbound connections only to network
 <net> (ipv4, ipv6,
       onion, i2p, cjdns). Inbound and manual connections 
are not
       affected by this option. It can be specified multip
le times to
       allow multiple networks.

  -peerblockfilters
       Serve compact block filters to peers per BIP 157 (d
efault: 0)

  -peerbloomfilters
       Support filtering of blocks and transaction with bl
oom filters (default:
       0)

  -port=<port>
       Listen for connections on <port> (default: 8333, te
stnet3: 18333,
       testnet4: 48333, signet: 38333, regtest: 18444). No
t relevant for
       I2P (see doc/i2p.md).

  -proxy=<ip:port|path>
       Connect through SOCKS5 proxy, set -noproxy to disab
le (default:
       disabled). May be a local file path prefixed with '
unix:' if the
       proxy supports it.

  -proxyrandomize
       Randomize credentials for every proxy connection. T
his enables Tor
       stream isolation (default: 1)

  -seednode=<ip>
       Connect to a node to retrieve peer addresses, and d
isconnect. This
       option can be specified multiple times to connect t
o multiple
       nodes. During startup, seednodes will be tried befo
re dnsseeds.

  -timeout=<n>
       Specify socket connection timeout in milliseconds. 
If an initial attempt
       to connect is unsuccessful after this amount of tim
e, drop it
       (minimum: 1, default: 5000)

  -torcontrol=<ip>:<port>
       Tor control host and port to use if onion listening
 enabled (default:
       127.0.0.1:9051). If no port is specified, the defau
lt port of
       9051 will be used.

  -torpassword=<pass>
       Tor control port password (default: empty)

  -upnp
       Use UPnP to map the listening port (default: 0)

  -v2transport
       Support v2 transport (default: 1)

  -whitebind=<[permissions@]addr>
       Bind to the given address and add permission flags 
to the peers
       connecting to it. Use [host]:port notation for IPv6
. Allowed
       permissions: bloomfilter (allow requesting BIP37 fi
ltered blocks
       and transactions), noban (do not ban for misbehavio
r; implies
       download), forcerelay (relay transactions that are 
already in the
       mempool; implies relay), relay (relay even in -bloc
ksonly mode,
       and unlimited transaction announcements), mempool (
allow
       requesting BIP35 mempool contents), download (allow
 getheaders
       during IBD, no disconnect after maxuploadtarget lim
it), addr
       (responses to GETADDR avoid hitting the cache and c
ontain random
       records with the most up-to-date info). Specify mul
tiple
       permissions separated by commas (default:
       download,noban,mempool,relay). Can be specified mul
tiple times.

  -whitelist=<[permissions@]IP address or network>
       Add permission flags to the peers using the given I
P address (e.g.
       1.2.3.4) or CIDR-notated network (e.g. 1.2.3.0/24).
 Uses the same
       permissions as -whitebind. Additional flags "in" an
d "out"
       control whether permissions apply to incoming conne
ctions and/or
       manual (default: incoming only). Can be specified m
ultiple times.

Wallet options:

  -addresstype
       What type of addresses to use ("legacy", "p2sh-segw
it", "bech32", or
       "bech32m", default: "bech32")

  -avoidpartialspends
       Group outputs by address, selecting many (possibly 
all) or none, instead
       of selecting on a per-output basis. Privacy is impr
oved as
       addresses are mostly swept with fewer transactions 
and outputs
       are aggregated in clean change addresses. It may re
sult in higher
       fees due to less optimal coin selection caused by t
his added
       limitation and possibly a larger-than-necessary num
ber of inputs
       being used. Always enabled for wallets with "avoid_
reuse"
       enabled, otherwise default: 0.

  -changetype
       What type of change to use ("legacy", "p2sh-segwit"
, "bech32", or
       "bech32m"). Default is "legacy" when -addresstype=l
egacy, else it
       is an implementation detail.

  -consolidatefeerate=<amt>
       The maximum feerate (in BTC/kvB) at which transacti
on building may use
       more inputs than strictly necessary so that the wal
let's UTXO
       pool can be reduced (default: 0.0001).

  -disablewallet
       Do not load the wallet and disable wallet RPC calls

  -discardfee=<amt>
       The fee rate (in BTC/kvB) that indicates your toler
ance for discarding
       change by adding it to the fee (default: 0.0001). N
ote: An output
       is discarded if it is dust at this rate, but we wil
l always
       discard up to the dust relay fee and a discard fee 
above that is
       limited by the fee estimate for the longest target

  -fallbackfee=<amt>
       A fee rate (in BTC/kvB) that will be used when fee 
estimation has
       insufficient data. 0 to entirely disable the fallba
ckfee feature.
       (default: 0.00)

  -keypool=<n>
       Set key pool size to <n> (default: 1000). Warning: 
Smaller sizes may
       increase the risk of losing funds when restoring fr
om an old
       backup, if none of the addresses in the original ke
ypool have
       been used.

  -maxapsfee=<n>
       Spend up to this amount in additional (absolute) fe
es (in BTC) if it
       allows the use of partial spend avoidance (default:
 0.00)

  -mintxfee=<amt>
       Fee rates (in BTC/kvB) smaller than this are consid
ered zero fee for
       transaction creation (default: 0.00001)

  -paytxfee=<amt>
       Fee rate (in BTC/kvB) to add to transactions you se
nd (default: 0.00)

  -signer=<cmd>
       External signing tool, see doc/external-signer.md

  -spendzeroconfchange
       Spend unconfirmed change when sending transactions 
(default: 1)

  -txconfirmtarget=<n>
       If paytxfee is not set, include enough fee so trans
actions begin
       confirmation on average within n blocks (default: 6
)

  -wallet=<path>
       Specify wallet path to load at startup. Can be used
 multiple times to
       load multiple wallets. Path is to a directory conta
ining wallet
       data and log files. If the path is not absolute, it
 is
       interpreted relative to <walletdir>. This only load
s existing
       wallets and does not create new ones. For backwards
 compatibility
       this also accepts names of existing top-level data 
files in
       <walletdir>.

  -walletbroadcast
       Make the wallet broadcast transactions (default: 1)

  -walletdir=<dir>
       Specify directory to hold wallets (default: <datadi
r>/wallets if it
       exists, otherwise <datadir>)

  -walletnotify=<cmd>
       Execute command when a wallet transaction changes. 
%s in cmd is replaced
       by TxID, %w is replaced by wallet name, %b is repla
ced by the
       hash of the block including the transaction (set to
 'unconfirmed'
       if the transaction is not included) and %h is repla
ced by the
       block height (-1 if not included). %w is not curren
tly
       implemented on windows. On systems where %w is supp
orted, it
       should NOT be quoted because this would break shell
 escaping used
       to invoke the command.

  -walletrbf
       Send transactions with full-RBF opt-in enabled (RPC
 only, default: 1)

ZeroMQ notification options:

  -zmqpubhashblock=<address>
       Enable publish hash block in <address>

  -zmqpubhashblockhwm=<n>
       Set publish hash block outbound message high water 
mark (default: 1000)

  -zmqpubhashtx=<address>
       Enable publish hash transaction in <address>

  -zmqpubhashtxhwm=<n>
       Set publish hash transaction outbound message high 
water mark (default:
       1000)

  -zmqpubrawblock=<address>
       Enable publish raw block in <address>

  -zmqpubrawblockhwm=<n>
       Set publish raw block outbound message high water m
ark (default: 1000)

  -zmqpubrawtx=<address>
       Enable publish raw transaction in <address>

  -zmqpubrawtxhwm=<n>
       Set publish raw transaction outbound message high w
ater mark (default:
       1000)

  -zmqpubsequence=<address>
       Enable publish hash block and tx sequence in <addre
ss>

  -zmqpubsequencehwm=<n>
       Set publish hash sequence message high water mark (
default: 1000)

Debugging/Testing options:

  -debug=<category>
       Output debug and trace logging (default: -nodebug, 
supplying <category>
       is optional). If <category> is not supplied or if <
category> is 1
       or "all", output all debug logging. If <category> i
s 0 or "none",
       any other categories are ignored. Other valid value
s for
       <category> are: addrman, bench, blockstorage, cmpct
block, coindb,
       estimatefee, http, i2p, ipc, leveldb, libevent, mem
pool,
       mempoolrej, net, proxy, prune, qt, rand, reindex, r
pc, scan,
       selectcoins, tor, txpackages, txreconciliation, val
idation,
       walletdb, zmq. This option can be specified multipl
e times to
       output multiple categories.

  -debugexclude=<category>
       Exclude debug and trace logging for a category. Can
 be used in
       conjunction with -debug=1 to output debug and trace
 logging for
       all categories except the specified category. This 
option can be
       specified multiple times to exclude multiple catego
ries. This
       takes priority over "-debug"

  -help-debug
       Print help message with debugging options and exit

  -logips
       Include IP addresses in debug output (default: 0)

  -loglevelalways
       Always prepend a category and level (default: 0)

  -logsourcelocations
       Prepend debug output with name of the originating s
ource location
       (source file, line number and function name) (defau
lt: 0)

  -logthreadnames
       Prepend debug output with name of the originating t
hread (default: 0)

  -logtimestamps
       Prepend debug output with timestamp (default: 1)

  -maxtxfee=<amt>
       Maximum total fees (in BTC) to use in a single wall
et transaction;
       setting this too low may abort large transactions (
default: 0.10)

  -printtoconsole
       Send trace/debug info to console (default: 1 when n
o -daemon. To disable
       logging to file, set -nodebuglogfile)

  -shrinkdebugfile
       Shrink debug.log file on client startup (default: 1
 when no -debug)

  -uacomment=<cmt>
       Append comment to the user agent string

Chain selection options:

  -chain=<chain>
       Use the chain <chain> (default: main). Allowed valu
es: main, test,
       testnet4, signet, regtest

  -signet
       Use the signet chain. Equivalent to -chain=signet. 
Note that the network
       is defined by the -signetchallenge parameter

  -signetchallenge
       Blocks must satisfy the given script to be consider
ed valid (only for
       signet networks; defaults to the global default sig
net test
       network challenge)

  -signetseednode
       Specify a seed node for the signet network, in the 
hostname[:port]
       format, e.g. sig.net:1234 (may be used multiple tim
es to specify
       multiple seed nodes; defaults to the global default
 signet test
       network seed node(s))

  -testnet
       Use the testnet3 chain. Equivalent to -chain=test. 
Support for testnet3
       is deprecated and will be removed in an upcoming re
lease.
       Consider moving to testnet4 now by using -testnet4.

  -testnet4
       Use the testnet4 chain. Equivalent to -chain=testne
t4.

Node relay options:

  -bytespersigop
       Equivalent bytes per sigop in transactions for rela
y and mining
       (default: 20)

  -datacarrier
       Relay and mine data carrier transactions (default: 
1)

  -datacarriersize
       Relay and mine transactions whose data-carrying raw
 scriptPubKey is of
       this size or less (default: 83)

  -mempoolfullrbf
       (DEPRECATED) Accept transaction replace-by-fee with
out requiring
       replaceability signaling (default: 1)

  -minrelaytxfee=<amt>
       Fees (in BTC/kvB) smaller than this are considered 
zero fee for
       relaying, mining and transaction creation (default:
 0.00001)

  -permitbaremultisig
       Relay transactions creating non-P2SH multisig outpu
ts (default: 1)

  -whitelistforcerelay
       Add 'forcerelay' permission to whitelisted peers wi
th default
       permissions. This will relay transactions even if t
he
       transactions were already in the mempool. (default:
 0)

  -whitelistrelay
       Add 'relay' permission to whitelisted peers with de
fault permissions.
       This will accept relayed transactions even when not
 relaying
       transactions (default: 1)

Block creation options:

  -blockmaxweight=<n>
       Set maximum BIP141 block weight (default: 3996000)

  -blockmintxfee=<amt>
       Set lowest fee rate (in BTC/kvB) for transactions t
o be included in
       block creation. (default: 0.00001)

RPC server options:

  -rest
       Accept public REST requests (default: 0)

  -rpcallowip=<ip>
       Allow JSON-RPC connections from specified source. V
alid values for <ip>
       are a single IP (e.g. 1.2.3.4), a network/netmask (
e.g.
       1.2.3.4/255.255.255.0), a network/CIDR (e.g. 1.2.3.
4/24), all
       ipv4 (0.0.0.0/0), or all ipv6 (::/0). This option c
an be
       specified multiple times

  -rpcauth=<userpw>
       Username and HMAC-SHA-256 hashed password for JSON-
RPC connections. The
       field <userpw> comes in the format: <USERNAME>:<SAL
T>$<HASH>. A
       canonical python script is included in share/rpcaut
h. The client
       then connects normally using the
       rpcuser=<USERNAME>/rpcpassword=<PASSWORD> pair of a
rguments. This
       option can be specified multiple times

  -rpcbind=<addr>[:port]
       Bind to given address to listen for JSON-RPC connec
tions. Do not expose
       the RPC server to untrusted networks such as the pu
blic internet!
       This option is ignored unless -rpcallowip is also p
assed. Port is
       optional and overrides -rpcport. Use [host]:port no
tation for
       IPv6. This option can be specified multiple times (
default:
       127.0.0.1 and ::1 i.e., localhost)

  -rpccookiefile=<loc>
       Location of the auth cookie. Relative paths will be
 prefixed by a
       net-specific datadir location. (default: data dir)

  -rpccookieperms=<readable-by>
       Set permissions on the RPC auth cookie file so that
 it is readable by
       [owner|group|all] (default: owner [via umask 0077])

  -rpcpassword=<pw>
       Password for JSON-RPC connections

  -rpcport=<port>
       Listen for JSON-RPC connections on <port> (default:
 8332, testnet3:
       18332, testnet4: 48332, signet: 38332, regtest: 184
43)

  -rpcthreads=<n>
       Set the number of threads to service RPC calls (def
ault: 4)

  -rpcuser=<user>
       Username for JSON-RPC connections

  -rpcwhitelist=<whitelist>
       Set a whitelist to filter incoming RPC calls for a 
specific user. The
       field <whitelist> comes in the format: <USERNAME>:<
rpc 1>,<rpc
       2>,...,<rpc n>. If multiple whitelists are set for 
a given user,
       they are set-intersected. See -rpcwhitelistdefault 
documentation
       for information on default whitelist behavior.

  -rpcwhitelistdefault
       Sets default behavior for rpc whitelisting. Unless 
rpcwhitelistdefault
       is set to 0, if any -rpcwhitelist is set, the rpc s
erver acts as
       if all rpc users are subject to empty-unless-otherw
ise-specified
       whitelists. If rpcwhitelistdefault is set to 1 and 
no
       -rpcwhitelist is set, rpc server acts as if all rpc
 users are
       subject to empty whitelists.

  -server
       Accept command line and JSON-RPC commands
