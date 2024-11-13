---
title: "The Total Cost of Unmasked Data"
date: "2014-02-12"
categories: 
  - "delphix"
tags: 
  - "masking"
---

[![](images/20140125_FNP004_0.jpg "20140125_FNP004_0")](http://ahl.dtrace.org/wp-content/uploads/2014/02/20140125_FNP004_0.jpg)Data breaches make headlines at a regular cadence. Each is a surprise, but they are not, as a whole, surprising. While the [extensive and sophisticated Target breach](http://www.forbes.com/sites/paularosenblum/2014/01/17/the-target-data-breach-is-becoming-a-nightmare/) stuck in the headlines, a significant breach at three South Korean credit card companies happened around the same time. The theft of personal information for 20m subscribers didn't have near the level of sophistication. Developers and contractors were simply given copies of production databases filled with personal information that they shouldn't have been able to access.

When talking to Delphix customers and prospects, those that handle personal or sensitive information (typically financial services or heath care) inevitably ask how Delphix can help with masking. Turning the question around, asking how they mask data today sucks the air out of the room. Some deflect, talking about relevant requirements and regulations; others, pontificate obliquely about solutions they’ve bought; **no one unabashedly claims to be fully implemented and fully compliant**.

Data masking is hard to deploy consistently. I hear it from (honest) customers, and from data masking vendors. The striking attribute of the South Korean breach was that the [Economist](http://www.economist.com/news/finance-and-economics/21595059-enormous-data-heist-may-dim-koreans-love-affair-credit-cards-card-sharps) and other non-technical news sources called out unmasked data as the root cause:

> “In 2012 a law was passed requiring the encryption of most companies’ databases, yet the filched data were not encoded. The contractor should never have been given access to customer records, he says; dummy data would have sufficed.”

These were non-production database copies, used for development and testing. There was no need for employees or contractors to interact with sensitive data. Indeed, those companies have a legal obligation **not** to keep production data in their development environments. **All three credit card companies, and the credit bureau are customers of vendors that provide masking solutions.** The contractor who loaded data for 20m individuals onto a USB stick didn’t need the real data, and should never been granted access. As with the customers I talk to, data masking surely proved too difficult to roll out in a manner that was secure and didn’t slow development, so it was relegated to shelfware.

Delphix fully automates the creation of non-production environments. It [integrates with masking tools](http://www.delphix.com/2013/12/data-masking-partnership/) from Axis, Informatica, IBM, and others to ensure that every one of those environments is masked as a matter of mechanism rather than a manual process. What is the cost of unimplemented data masking? Obviously there are the fines and negative press, the lawsuits, and the endless mea culpas. At these credit card companies though [literally dozens of executives resigned for failing to secure data](http://www.bbc.co.uk/news/technology-25808189), from all three CEOs on down. And in all likelihood, they had data masking solutions on the shelf, cast aside as too hard to implement.
