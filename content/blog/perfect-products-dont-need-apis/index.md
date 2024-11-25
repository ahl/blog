---
title: "Perfect products don’t need APIs (the rest do)"
date: "2019-04-12"
permalink: /google-hire-has-an-api-finally-81c99b11de83/
---

We had just convinced our Seed round investors that Transposit was a big idea. A big idea needs a team to build it, so when the first round closed we started hiring that team. We’d talk to candidates and write down our feedback to share with the team. At first we used Google Docs. The price was right (free), but we quickly outgrew it: we needed more structure and access control. My experience with application tracking systems (ATS) is that they’re all infuriating in their own way, imposing their structure on your process rather than adapting to it. So if we weren’t going to get what we wanted, at least we didn’t want to pay a lot for it.

Enter Google Hire.

<figure>
  <img src="images/bosch.jpg" width="100%" alt="" class="hero-image"></img>
  <figcaption>Hell is other people... trapping your data with no API</figcaption>
</figure>

As anticipated, Google Hire wasn’t everything what we wanted. What it was though was cheap. It’s not terrible; it just doesn’t do anything quite the way we’d like. Filter by recruiter, handoff candidate ownership, find folks we haven’t pinged in awhile — Hire didn’t do any of those things.

No problem, right? That’s what APIs are for. **Your service might not suit my needs, but an API lets me work around the parts I hate and build the extensions I need.** Even if the product improved to meet our needs, there would always be a next need. But — amazingly for a Google product\[1\] — there was no API.

## All product need APIs

No product is perfect for all users. APIs are the escape hatch. They’re the panacea for any complaint. They’re the answer for the long tail of features you’ll never ever get to.

With no API for Google Hire, we just suffered. And in the meantime we hired our team and [built our product](https://www.transposit.com) that — with some irony — made it easy to work with APIs. By a happy coincidence, Google Hire released their [beta API](https://developers.google.com/hire/tenants/guides/getting-started) right around the time Transposit opened our beta. Transposit puts a SQL and JS interface in front of APIs, manages auth, and makes it easy to build API-driven APIs. Here are the first couple of queries I ran against the Google Hire API. (You can [try everything out live](https://console.transposit.com/t/ahl/hire) once Google gives you API access.)

I can list all the candidates we’re tracking:

```sql
SELECT * FROM google_hire.list_candidates
  WHERE tenant=‘my_tenant’
```

I can see the past employers of all the folks we’re talking to:

```sql
SELECT employmentInfo.employer FROM google_hire.list_candidates
  WHERE tenant=’my_tenant’ EXPAND BY employmentInfo   `
```

(Amusingly in the context of this post, the top hit is Google)

## Solving problems

After being held at arm’s length for far too long, I had a lot of fun exploring the newly public(-ish\[2\]) API. The first real problem I wanted to solve was to show the candidates whom we haven’t pinged recently. Why is this not built in? I have no idea. It was easy enough to get to with Transposit. We treat each API like a view or table within a database; getting the data I wanted amounted to a JOIN between the candidates and applications tables:

<script src="https://gist.github.com/ahl/3195700ad377dc24bbcfd79693cb67a9.js"></script>

I can focus that in on a particular job by adding another SQL filter:

<script src="https://gist.github.com/ahl/ef1f731d5c7a2645f35c367d17dbfb7f.js"></script>

… and then get the data I want by plucking out particular fields and adding an ORDER BY:

<script src="https://gist.github.com/ahl/0454e83c8c460b1049d26a103b260961.js"></script>

```json
[
  {
    "name": "tenants/abc/candidates/def",
    "job": "tenants/acb/jobs/ghi",
    "candidate": "tenants/abc/candidates/jkl",
    "createTime": "2019-04-02T00:31:48.630Z",
    "status": {
      "state": "ACTIVE",
      "processStage": {
        "stage": "Initial Screen",
        "reportingCategory": "SCREEN"
      },
      "updateTime": "2019-04-03T05:27:29.321Z"
    },
    "personName": {
      "givenName": "David",
      "familyName": "Lightman"
    },
...
```

Finally! The data I’ve wanted since we first started using Hire!

## All Hail Our Benevolent Slack Overlords

Slack is rapidly becoming the central interface across so many tools we use, so I wanted the data from Google Hire connected up with Slack. Building a Slack slash command is a snap ([check out the Transposit Quickstart](https://docs.transposit.com/get-started/quickstart)). You just need to point Slack at a webhook that returns the data you want:

Now when I worry if we’ve dropped the ball on a candidate and they haven’t heard from us in a while, I can just type my Slack slash command:

## Next

As excited as I was for the Google Hire API, the endpoints in the first beta are pretty basic. It will be extremely powerful when they open up the ability to do things like create new candidates, transition candidates between stages of the hiring process, and add comments (the mechanism we use to hand off candidates between members of our team).

An API is a critical feature of any tool we use; **I’ll never again make the mistake of choosing a tool that doesn’t give me access to my data via an API**. Transposit let me explore the Google Hire API interactively, check out the data, and poke around at different endpoints. You can [try it out](http://www.transposit.com) with the Hire API or any of the other APIs it connects to. And if this sounds like something you’d like to get involved in, [we’re hiring](https://www.transposit.com/jobs/) (and we’re good at follow up).

```sql
SELECT title FROM google_hire.list_jobs WHERE tenant=’my_tenant’ AND filter=’state=OPEN’   `
```

```json
[
  {
    "title": "Developer Advocate"
  },
  {
    "title": "Growth Marketing Manager"
  },
  {
    "title": "Product Marketing Manager"
  },
  {
    "title": "Senior Site Reliability Engineer"
  },
  {
    "title": "Software Engineer"
  }
]
```

\[1\] I call it a Google Product — and I knew this beforehand — but [Hire came from the Bebop acquisition that brought Diane Greene into Google](https://venturebeat.com/2017/04/13/google-hire-is-a-job-site-that-diane-greenes-bebop-has-been-quietly-working-on/). I suspect acquiring an amazing leader at the head of GCP may have been the primary goal.

\[2\] You need to [sign up and get approved for the Google Hire API](https://developers.google.com/hire/tenants/guides/getting-started). I wrote a pleading, but firm mail to our sales rep to nudge things along.
