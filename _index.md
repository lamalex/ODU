+++
title = "Travis et al: 3rd party CI services"
outputs = ["Reveal"]

[reveal_hugo]
slide_number = true
theme = "solarized"
#custom_theme = "theme-overrides.scss"
#custom_theme_compile = true
+++

# Travis et al
## 3rd party CI/CD services

> Alex Launi <br />
> CS795 Fall 2020

{{% fragment %}} *`Built and deployed with Travis CI`* {{% /fragment %}}

{{% note %}}
* Introduce self
* Establish relevancy
* Establish SME

And for the record: this presentation is built and deployed using Travis!
{{% /note %}}

---

## Objectives

* LIST CI/CD services
* DESCRIBE Travis CI system architecture
* INTERPRET common `.travis.yml` verbs
* DEPLOY a static site to GitHub pages via Travis

---

## References

* [Official Travis Docs](https://docs.travis-ci.com/)
* [Official CircleCI Docs](https://circleci.com/docs/2.0/)
* [Official Azure Pipelines Docs](https://docs.microsoft.com/en-us/azure/devops/pipelines/?view=azure-devops)

---

## CI is *fundamental* to DevOps

{{% note %}}
At the heart of modern software delivery is a fast, reliable, and automated feedback cycle.
Developers need to be able to quickly share, iterate, and test their work.
Out of this need a market has arrisen to provide this systems as a service.

As we've covered over this semester decreasing the resistance to deploying software is a *competitive advantage*.
As demonstrated by the proliferation of CI/CD services solving build/deploy is a product in and of itself.
Spend your time on your central line of business, and delivering quality. Let someone else solve the build problem.

In 2020 even the simple projects deserve their own CI pipeline. Even as students this has value. Professors/PIs,
although intelligent and experts in their field are busy, and in my experience need access to your project to be
low friction.

Anecdotal evidence: In my work in Dr. Jain's lab I spent a few weeks developing an application for a study we are
conducting. Despite our weekly meetings and on-screen demos, when I finally set up a deployment pipeline and Dr.
Jain was able to test the app on her own hardware we realized I had very much missed the mark.

We needed significant iteration on the design, but now I had an automated build/deployment pipeline. I was able to
get updates into her hands **effortlessly**. 

I should have set this up **first**. 
{{% /note %}}

---

## Infrastructure is expensive both in *time* and *dollars*

{{% note %}}
But why use a 3rd party service?

* On-site equipment has administrative costs- electricity, a technician/administrator, etc.
  * Out-sourcing CI/CD infrastructure allows you to focus on building your product, and not managing a build system.

I could have set up an XCode build server in my house to do the same thing that Travis provided, but I would have
had to spend time managing it.

And what about when requirements change? For instance I have a message queue that collects data from the mobile app.
I would need to spend time configurating a manual build/deploy system for that Rust project.

A 3rd party CI/CD allows you to employ a single ecosystem for build/deploy across languages. 
{{% /note %}}

---

## There are many CI/CD services that you can use
* [Travis CI](https://travis-ci.com/)
* [Circle CI](https://circleci.com)
* [Azure Pipelines](https://azure.microsoft.com/en-us/services/devops/pipelines/)
* [TeamCity](https://www.jetbrains.com/teamcity/)
* [Jenkins](https://www.jenkins.io)
* [and many more!](http://google.com/search?q=list+of+ci%2Fcd)

{{% note %}}
We will be focusing on `Travis CI`, but much of the information will be applicable to other CI/CD services
For instance: yaml is used to configure Travis, Circle, Github Actions, GitLab CI, and Azure Pipelines
{{% /note %}}

---

so let's talk about
# Travis CI

{{% note %}}
As students we have access to the GitHub student developer pack, which among many other great things,
gives you access to unlimited private builds in Travis. If you have not yet requested your developer pack
I **HIGHLY** recommend you do that. It's extraordinarily valuable.
{{% /note %}}

---

## Build Platforms

<div style="font-size: 20px;" >

| Build OS | Infrastructure | CPU Archs | CPU Cores | Memory | 
|:---------|----------------|-----------|:---------:|-------:|
| Ubuntu   | <ul><li>GCE/AWS VM</li></ul> | <ul><li>amd64</li><li>arm64-graviton2</li></ul> | 2 | 7.5GB |
| Ubuntu (LXD) | <ul><li>ARM: Equinix, AWS</li><li>IBM POWER/Z: IBM Cloud</li></ul><br /> | <ul><li>arm v8</li><li>arm64-graviton2</li><li>PPC64</li><li>IBM z</li></ul> | 2 | ~4GB |
| macOS    | <ul><li>VM</li></ul><br /> | <ul><li>amd64</li></ul> | 2 | 4GB |
| Windows Server  | <ul><li>GCE VM</li></ul><br/> | <ul><li>amd64</li></ul> | 2 | 8GB |

</div>


{{% note %}}
Travis offers 3 build operating systems.
Travis is built around virtualization, but takes a hybrid approach to where those VMs run.

AWS, Google Cloud, IBM Cloud, and on-prem HW provide high flexibility and availability

It is important to note that this is not the list of supported build targets for your project, this is simply
the hardware/platform you will build *on*. 

For instance, you could target an embedded Arduino with your project, but build **on** an Ubuntu 20.04 host.
Some targets require specific hosts. Software which targets any of the Apple systems need to be built on macOS due to
tool chain availability and licensing constraints.
{{% /note %}}

---

{{% section %}}
# Builds, Jobs, Phases, and Stages

{{% note %}}
A build in Travis CI is a sequence of stages. Each stage consists of jobs run in parallel.

- The highest level unit of work in Travis is the `Build`.
- A build is a group of `jobs` that run in sequence.
- A job is built from sequential steps: `phases`
- `Stages` allow you to group jobs within a build
{{% /note %}}

---

# Build
Travis uses a *delcarative* format for specifying how your project should be built

```yaml
# Specifying language will install that language's toolchain
language: elm
# you can specify a specific version of your toolchain
env:
  - elm0.19.0
# or which underlying OS you want to build on
os: 20.04
# and which ISA the VM targets
arch: amd64

...
```

<div style="font-size: 20px; text-align: left;">
Travis supported languages are
Android
C
C#
C++
Clojure
Crystal
D
Dart
Elixir
Elm
Erlang
F#
Generic
Go
Groovy
Haskell
Haxe
Java
JavaScript
Julia
MATLAB
Minimal
Nix
Objective-C
Perl
Perl6
PHP
Python
R
Ruby
Rust
Scala
Smalltalk
Swift
Visual Basic
</div>

{{% note %}}
The .travis.yml file describes the build process. 
Yaml stands for
> Yaml aint markup language

It's a simple, easy to read serialization format that has become popular and you will come across
more as you do more devops (for instance with kubernetes)

Travis build are usually triggered by a commit to a source control hosting platform
 - GitHub
 - GitLab
 - BitBucket
 - Assembla

but can also be **manually triggered** or **scheduled via cron**
{{% /note %}}

---

## A job is a **sequence** of *phases*

{{% note %}}
Job phases are run in serial and do everything from installing additional dependencies to deployment
{{% /note %}}

---

## Phases
```yaml
...

before_install:
install: # install any dependencies
before_script:
script: # this is the build phase
before_cache: #only used if caching was enabled

# Only 1 of these will run per stage
after_success:
after_failure:

before_deploy:
deploy:
after_deploy:
after_script:
```

{{% note %}}
Job steps are configured and run in the order shown here.
Dependencies can be installed with various package managers like apt, homebrew, npm, cargo, etc.
Arbitrary shell commands can be issued so these phases can be used flexibly, rather than adhering to
a rigid semantic meaning.
{{% /note %}}

---

## Stages

Stages group jobs to be run in parallel, but run each stage sequentially
![Travis stages in action](images/stages.gif)

{{% note %}}
Stages are a powerful and flexible tool.
In the animation a Ruby project is executing 2 concurrent jobs on different versions of Ruby, 
and only if **both** jobs pass will the deploy stage execute.
{{% /note %}}

---

## Stage usage

```yaml
...

jobs:
  include:
    # Stages will run sequentially
    - stage: test
      # scripts will run concurrently on separate VMs
      script: ./test 1
      script: ./test 2
    - stage: deploy
      script: ./deploy
```

{{% /section %}}

--- 

# Let's deploy!

{{% note %}}
In this section we're going to deploy a snake game as a static site to GitHub pages.
You'll need, and should already have:
- git installed
- a GitHub account
{{% /note %}}

---

## Let's setup GitHub
<div style="text-align: left;" >

1) Fork the [Snake repository](https://github.com/lamalex/cs795-elm) (lamalex/cs795-elm)
2) Enable GitHub pages in `Settings`
   1) Set `Source` to `gh-pages` `/ (root)`
3) [Create a new API token](https://github.com/settings/tokens/new) with `repo` scope
   - Keep this available, you'll need it and it's not recoverable - you'll have to generate a new one otherwise

</div>

---

## [Create an account on Travis-CI](https://travis-ci.com/signup)
  
![Sign up with github](images/signup-w-gh.png)

Then go to settings and activate GitHub

---

## Setup your deploy token

- Click settings on the CS795 repository
- Add an environment variable named `GH_TOKEN` to the value of token you generated in GitHub earlier on branch `main`. Do not display its value in build log

---

{{% section %}}

## Add a .travis.yml file

Use what you've learned in this lesson to write a travis configuration file. 


```bash
# 1) Run the tests with
$ elm-test
# 2) and build your site with   
$ elm make --optimize --output=public/index.html src/Main.elm
``` 

```
Small hint: 
    The default node.js version doesn't work with elm v0.19. 
    Set node_js: '15' to select a compatible node version
```

---

### Yar! There be dragons (spoilers) ahead 🐉

Please try write a config on your own. The slide below contains the steps to complete a full working deployment.

---

```yaml
dist: focal
language: elm
node_js: '15'

install:
  - npm i --also=dev

script:
  - elm-test
  - elm make --optimize --output=public/index.html src/Main.elm

deploy:
  provider: pages
  skip_cleanup: true
  local_dir: public
  token: $GH_TOKEN
  on:
    branch:
      - main
```

{{% /section %}}

---

## Oh snap!!
### Something went wrong

```
↓ SnakeTests
↓ Verify states map to a loss
✗ Playing is not a loss

    False
    ╷
    │ Expect.equal
    ╵
    True

TEST RUN FAILED
```

---

{{% section %}}

## Challenge 2
### Fix the test and push

---

### 🚨 Spoiler Alert!
The next slide contains a patch which will fix the failing test

---

```patch
cat <<EOF | git apply -
---
 tests/SnakeTests.elm | 2 +-
 1 file changed, 1 insertion(+), 1 deletion(-)

diff --git a/tests/SnakeTests.elm b/tests/SnakeTests.elm
index be0d327..fe4ff67 100644
--- a/tests/SnakeTests.elm
+++ b/tests/SnakeTests.elm
@@ -65,7 +65,7 @@ gameStateLossTests =
         [ test "Playing is not a loss" <|
             \_ ->
                 isLost Playing
-                    |> Expect.equal True
+                    |> Expect.equal False
         , test " Lost is a loss" <|
             \_ ->
                 isLost (Lost "Anything honestly")
EOF
```

Paste this in the project root directory to apply the patch which will fix the failing test.

{{% /section %}}

---

# You did it!
visit https://\<your-github-username\>.github.io/cs795-elm/ and play some snake!
