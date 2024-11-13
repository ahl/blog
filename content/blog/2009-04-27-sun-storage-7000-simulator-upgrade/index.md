---
title: "Sun Storage 7000 simulator upgrade"
date: "2009-04-27"
categories: 
  - "fishworks"
---

![](images/photo_virtual.png)

Today we released [version 2009.Q2.0.0](http://blogs.sun.com/fishworks/entry/sun_storage_7000_2009_q2), the first major software update for the Sun Storage 7000 series. It includes a bunch of new features, bug fixes, and improvements. Significantly for users of the [Sun Storage 7000 simulator](http://dtrace.org/blogs/ahl/fishworks_vm), the virtual machine version of the 7000 series, this is the first update that supports the VMs. As with a physical 7000 series appliance, upgrade by navigating to Maintenance > System, and click the **+** icon next to Available Updates. Remember not to ungzip the update binary — the appliance will do that itself. We'll be releasing an update VM preinstalled with the new bits so stay tuned.

**Note:** There were actually two releases of the VMware simulator. The first one came right around our initial launch, and the version string is `ak-2008.11.07`. This version cannot be upgraded so you'll need to download the updated simulator whose version is `ak-2008.11.21`. As noted above, we'll soon be releasing an updated VM with 2009.Q2.0.0 (`ak-2009.04.10.0.0`) preinstalled.
