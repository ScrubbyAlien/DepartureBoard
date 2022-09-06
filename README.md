To use this project you must have a valid api key from [trafiklab.se](https://trafiklab.se/) for the [ResRobot v2.1 API](https://www.trafiklab.se/api/trafiklab-apis/resrobot-v21/).

In your local repo create the config/ directory and create a config.toml file there.
Then add this to the .toml file:

    [keys]
    res_robot = 'YOUR_API_KEY'

Where &nbsp;*YOUR_API_KEY*&nbsp; is your api key.

To start up the server simply navigate to the projekt root and run `cargo run`. Then navigate to `localhost:8000/index.html` in your favorite browser to the see the project in action. 