+++
title = "Travis et al: 3rd party CI services"
outputs = ["Reveal"]

[reveal_hugo]
slide_number = true
#theme = "serif"
custom_theme = "theme-overrides.scss"
custom_theme_compile = true

+++

# Travis et al
## 3rd party CI/CD services

> Alex Launi <br />
> CS795 Fall 2020


{{% note %}}
* Introduce self
* Establish relevancy
* Establish SME
{{% /note %}}

---

## Objectives

* LIST CI/CD services
* DESCRIBE Travis CI system architecture
* INTERPRET common `.travis.yml` verbs
* DEPLOY a static site to GitHub pages via Travis

---

## Infrastructure is expensive both in *time* and *dollars*

{{% note %}}
* On-site equipment has administrative costs- electricity, a technician/administrator, etc.
  * Out-sourcing CI/CD infrastructure allows you to focus on building your product, and not managing a build system.

As we've covered over this semester decreasing the resistance to deploying software is a *competitive advantage*.
As demonstrated by the proliferation of CI/CD services solving build/deploy is a product in and of itself.
Spend your time on your central line of business, and delivering quality. Let someone else solve the build problem.
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
