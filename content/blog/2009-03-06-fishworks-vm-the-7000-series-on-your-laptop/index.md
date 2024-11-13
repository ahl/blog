---
title: "Fishworks VM: the 7000 series on your laptop"
date: "2009-03-06"
categories: 
  - "fishworks"
---

![](images/setup_virtual.png)

In May of 2007 I was lined up to give my first customer presentation of what would become the [Sun Storage 7000 series](http://www.sun.com/storage/disk_systems/unified_storage/). I inherited a well-worn slide deck describing the product, but we had seen the reactions of prospective customers who saw the software live and had a chance to interact with features such as [Analytics](http://blogs.sun.com/bmc/entry/fishworks_now_it_can_be); no slides would elicit that kind of response. So with some tinkering, I hacked up our installer and shoe-horned the prototype software into a virtual machine. The live demonstration was a hit despite some rocky software interactions.

![](images/hw_virtual.png)

As the months passed, our software became increasingly aware of our hardware platforms; the patches I had used for the virtual machine version fell into disrepair. Racing toward the product launch, neither I nor anyone else in the Fishworks group had the time to nurse it back to health. I found myself using months old software for a customer demo — a useful tool, but embarrassing given the advances we had made. We knew that the VM was going to be great for presentations, and we had talked about releasing a version to the general public, but that, we thought, was something that we could sort out after the product launch.

In the brief calm after the frenetic months finishing the product and just a few days before the [launch in Las Vegas](http://blogs.sun.com/fishworks/entry/fishworks_launch_event), our EVP of storage, [John Fowler](http://channelsun.sun.com/video/john+fowler+at+intel+dev+forum/1719845973), paid a visit to the Fishworks office. When we mentioned the VM version, his eyes lit up at the thought of how it would help storage professionals. Great news, but we realized that the next few days had just become much busier.

![](images/photo_virtual.png)

Creating the VM version was a total barn-raising. Rather than a one-off with sharp edges, adequate for a canned demo, we wanted to hand a product to users that would simulate exactly a Sun Storage 7000 series box. In about three days, everyone in the group pitched in to build what was essentially a brand new product and platform complete with a hardware view conjured from bits of our actual appliances.

After a frenetic weekend in November, the Sun Unified Storage Simulator was ready in time for the launch. You can download it [here](http://www.sun.com/storage/disk_systems/unified_storage/resources.jsp) for VMware. We had prepared versions for VirtualBox as well as VMware, preferring VirtualBox since it's a Sun product; along the way we found some usability issues with the VirtualBox version — we were pushing both products beyond their design center and VMware handled it better. Rest assured that we're working to resolve those issues and we'll release the simulator for VirtualBox just as soon as it's ready. Note that we didn't limit the functionality at all; what you see is exactly what you'll get with an actual 7000 series box (though the 7000 series will deliver much better performance than a [laptop](http://blogs.sun.com/bmc/entry/moore_s_outlaws)). Analytics, replication, compression, CIFS, iSCSI are all there; give it a try and see what you think.
