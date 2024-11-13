---
title: "Webex utilities"
date: "2012-09-14"
categories: 
  - "delphix"
tags: 
  - "mac-os-x"
  - "webex"
---

I wish that none of our customers encountered problems with our product, but they do, and when they do our means for remotely accessing their systems is often via a Webex shared screen. We remotely control their Delphix server to collect data (often using DTrace). While investigating a customer issue recently I developed a couple of techniques to work around common problems; I thought I'd share them in case others have similar problems -- and as a note to my future self who will certainly forget the specifics next time.

### Copying and Pasting

Webex makes it fairly easy to copy text from the remote system and paste it locally: just select the text, and that implicitly copies it to the clipboard. I do this very very often as I write DTrace scripts to collect data, and then want to record both the script and the output. To that end, the Mac OS X pbpaste(1) utility is unbelievably helpful; pbpaste emits the contents of the clipboard. For example, I'll select text in the webex and use pbpaste like this:

```
$ pbpaste | tee -a data.log
```

Doing that, I can both verify that I selected the right data, and append it to the log of all data collected. Sometimes, though, the remote data is annoying to copy because I need to scroll up -- the mouse latency over webex can make this an exasperating experience. In those cases where the text I want to transfer is longer than a page, I do the following on the remote system:

```
$ cat output | gzip -9c | uuencode /dev/stdin
begin 644 /dev/stdin
M'XL(`..C4E`"`]5:W7_;-A!_#Y#_@>@P),&0A,<O5=X2=&LWH`_M]K`^%9TK
M2THBU+8\24[3C^UO'TG%L2D1,B6[0ZJG0+[[Z7CWN^,=PRQ-BZ?+?#:-%G<P
...
```

I then select the text, and back on my mac do this to dump out the data:

```
$ pbpaste | uudecode -o /dev/stdout | gzip -cd
```

By compressing and uuencoding the data, even large chunks of output easily fit on one screen. Here are the results on a large-ish chunk of data I copied from a customer system:

```
$ cat customer.data.txt | wc -l
 234
$ cat customer.data.txt | gzip -9c | uuencode /dev/stdin | wc -l
 44
```

234 lines would have had me tearing my hair out as I tried to capture the output, scrolling backward with 250ms screen refresh latency; 44 lines wasn't bad at all. Depending on the exact text I seem to get an 80-90% reduction in lines to copy. Many thanks to Brendan Gregg who had mentioned this technique to me; I hadn't appreciated it fully until I absolutely needed it.

### Screen Savers v. Thinking/Lunch

When diagnosing a problem on a customer system, we like to be as unobtrusive as possible, so it's annoying when we need to disturb the customer to enter his or her password because the screen lock has kicked in while I'm thinking about the next step in the investigation, or I'm getting something to eat. Many enterprise environments make it such that the screen saver delay can't be changed. I spent a day a couple of weeks ago bringing my laptop to meetings, and running to get lunch (and elsewhere) so that I could move the mouse at least every 15 minutes.

\[youtube\_sc url="3JwiDkibkGM" width="224" class="alignright"\]I didn't want to modify the customer system ("I let you remotely access my computer, and you're installing what?!"). Instead I wanted to programmatically move the mouse every so often on my system to ensure the remote system wouldn't lock the screen. I couldn't find anything pre-fab, but thanks to the tips at stackoverflow, I pieced something together that wiggles the cursor around if it hasn't moved in a little while. I could post it compressed and uuencoded in keeping with the theme above (it's just 17 lines!), but instead I've added a github repo: [github.com/adamleventhal/wiggle](https://github.com/adamleventhal/wiggle).

### Happy Webex-ing

I hope people find these tips useful. Given my penchant for looking up past tips on my own blog, I'm sure at least my future self will be thanking me at some point...
