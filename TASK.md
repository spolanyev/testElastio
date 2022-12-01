# Elastio Rust Test Task

Thanks for your interest in becoming a member of the Elastio Rust team!  You have the option to submit a CV without a test task for our review and consideration, however if you don't have much relevant experience or if you just want to stand out among the crowd, you can also submit a test task to show us what you're capable of.  We review every submission, even if you have no programming experience and no university degree.  However, please carefully review the text of the vacancy to which you are applying, and make sure you satisfy the criteria like geographical location and legal work authorization before applying.

## Description

Assume you are writing a new commercial application from scratch. It is a weather CLI for Windows/Linux/macOS, which is responsible for showing weather to a user. The target consumer of the CLI is a human.

The CLI must be written in Rust. You are free to use any layout, dependencies and means to organize the project. It should be designed close to a real product that you would develop and evolve with a team.

The CLI must support at least the following two commands. Feel free to add anything more than that, or deviate from the requirements provided you explain the reasoning behind that.

The application must support at least two weather providers e.g. two different REST/gRPC/GraphQL/etc. APIs.

## Minimum Required Commands
---

Configure credentials for `<provider>`, where `<provider>` is some short name for a concrete weather API.

This command should interactively read the required configuration from the terminal,
and save it somewhere, to be able to authenticate with the weather API without
asking the user to provide credentials each time.

```bash
weather configure <provider>
```

---

Show weather for the provided `<address>`.

The date parameter is optional, None means now.

The command needs to know what `<provider>` to use when querying the weather. Deciding how the user should select the `<provider>` is up to you. Just keep in mind that users will not fequently change the `<provider>` they use.

The output should be in some arbitrary human-readable form.


```bash
weather get <address> [date=now]
```

## Providers

You can use any providers you choose for this task.  The following links may help to find suitable providers:

* [OpenWeather](https://openweathermap.org)
* [weatherapi](https://www.weatherapi.com)
* [AccuWeather](https://developer.accuweather.com)
* [AerisWeather](https://www.aerisweather.com)

It's very important that you implement at least two providers.  The abstractions required to support selecting a provider at runtime are a key part of this task.  If you implement only a single hard-coded provider the task will not be considered completed.

## Best Practices

The purpose of this task is to provide us with a demonstration of your capabilities, your thought processes, how you solve certain problems, and your skills as a software engineer.  Therefore, when implementing this task, consider it an audition for a professional software engineering position, and work accordingly.  In particular we'll be looking at how you approach testing the solution, how you organize the code, your attention to details like error handling (a bunch of `unwrap()` calls are not what we're looking for), how readable it is and how you approach documenting your code.

## Hints to Maximize Impact

To really impress us, you can do some or all of the following optional things:

* Make sure your code complies cleanly with `cargo clippy`
* Format your code with `rustfmt`
* Have a `README.md` in the repo which describes your project at the high level, and how a user can use it without having to read the code to figure it out.
* Use third-party crates where possible to avoid doing unnecessary work yourself
* Try to auto-generate crate docs with `cargo doc` and place doc comments as needed so that the generated docs do a good job explaining the public interfaces and how they work.
* Use GitHub Actions or some other CI/CD system to compile your code on Windows, Mac, and Linux automatically when you merge.

## Submitting Your Task

You should perform this task in a public GitHub or GitLab repo.  Once you have finished it and are ready for us to review, send an email to [rust-jobs@elastio.com](mailto:rust-jobs@elastio.com).  In the subject line, write "Completed Test Task".  In the body of the message, provide the following:

* The URL to the repo where you published your completed task.
* A link to the vacancy to which you are applying.

Please include your CV in the email as an attachment, or include a link to your CV in the message body.

Be careful to ensure you have followed the instructions above, or your application could be delayed or rejected.

We try to reply to all applicants within one week, but depending on volume our team may take longer to get back to you.