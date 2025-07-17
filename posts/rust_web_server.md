---
title: Creating a blog site with a Rust Web-Server
tagline: This website was written in Rust.
tags: [Rust, Website, OpenSource]
---
When I sat down to create this portfolio, I knew I wanted to implement my own blog to document my projects and open-source contributions.
I looked into using popular web frameworks such as Next.JS or Vite, but sometimes, the best solution is the easiest one.

## Project Requirements
A software developer's blog gives you a look into their thoughts and implementation details on their projects. I think blogs are a great resource for the community. That's why it was important to me that my blog posts could be indexed by search engines. To do this, I knew I would prefer to use Server-Side Rendering instead of static HTML files, so that my posts could still be indexed without sacrificing templating.