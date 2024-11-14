---
title: "iSCSI DTrace provider and more to come"
date: "2007-07-03"
categories: 
  - "dtrace"
---

People often ask about the future direction of DTrace, and while we have some stuff planned for the core infrastructure, the future is really about extending DTrace's scope into every language, protocol, and application with new providers -- and this development is being done by many different members of the DTrace community. An important goal of this new work is to have consistent providers that work predictably. To that end, [Brendan](http://blogs.sun.com/brendan/) and I have started to [sketch out an array of providers](http://www.solarisinternals.com/wiki/index.php/DTrace_Topics) so that we can build a consistent model.

In that vein, I recently integrated a provider for our [iSCSI target](http://opensolaris.org/os/project/iscsitgt/) into Solaris Nevada (build 69, and it should be in a Solaris 10 update, but don't ask me which one). It's an [USDT provider](http://dtrace.org/blogs/ahl/user_land_tracing_gets_better) so the process ID is appended to the name; you can use `\*` to avoid typing the PID of the iSCSI target daemon. Here are the probes with their arguments (some of the names are obvious; for others you might need to refer to the iSCSI spec):

| probe name | args\[0\] | args\[1\] | args\[2\] |
| --- | --- | --- | --- |
| iscsi\*:::async-send | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::login-command | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::login-response | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::logout-command | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::logout-response | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::data-receive | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::data-request | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::data-send | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::nop-receive | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::nop-send | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::scsi-command | conninfo\_t \* | iscsiinfo\_t \* | iscsicmd\_t \* |
| iscsi\*:::scsi-response | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::task-command | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::task-response | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::text-command | conninfo\_t \* | iscsiinfo\_t \* | \- |
| iscsi\*:::text-response | conninfo\_t \* | iscsiinfo\_t \* | \- |

The argument structures are defined as follows:

```
typedef struct conninfo {
string ci_local;        /* local host address */
string ci_remote;       /* remote host address */
string ci_protocol;     /* protocol (ipv4, ipv6, etc) */
} conninfo_t;
typedef struct iscsiinfo {
string ii_target;               /* target iqn */
string ii_initiator;            /* initiator iqn */
uint64_t ii_lun;                /* target logical unit number */
uint32_t ii_itt;                /* initiator task tag */
uint32_t ii_ttt;                /* target transfer tag */
uint32_t ii_cmdsn;              /* command sequence number */
uint32_t ii_statsn;             /* status sequence number */
uint32_t ii_datasn;             /* data sequence number */
uint32_t ii_datalen;            /* length of data payload */
uint32_t ii_flags;              /* probe-specific flags */
} iscsiinfo_t;
typedef struct iscsicmd {
uint64_t ic_len;        /* CDB length */
uint8_t *ic_cdb;        /* CDB data */
} iscsicmd_t;

```

Note that the arguments go from most generic (the connection for the application protocol) to most specific. As an aside, we'd like future protocol providers to make use of the `conninfo\_t` so that one could write a simple script to see a table of frequent consumers for all protocols:

```
iscsi*:::,
http*:::,
cifs:::
{
@[args[0]->ci_remote] = count();
}

```

With the iSCSI provider you can quickly see which LUNs are most active:

```
iscsi*:::scsi-command
{
@[args[1]->ii_target] = count();
}

```

or the volume of data transmitted:

```
iscsi*:::data-send
{
@ = sum(args[1]->ii_datalen);
}

```

Brendan has been working on [a bunch of iSCSI scripts](http://www.solarisinternals.com/wiki/index.php/DTrace_Topics_iSCSI) -- those are great for getting started examining iSCSI
