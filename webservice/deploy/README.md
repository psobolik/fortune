# Deploy Fortune API
To deploy the Fortune API on an Apache server, specifically `marconi`.

1. Copy the data files to the server [^1]. 
1. Install the application files in `/srv/fortuneapi`

   | File          | Description                                              |
   |---------------|----------------------------------------------------------|
   | `fortuneapi`  | The executable                                           |
   | `Config.toml` | This tells the program where to find the data files[^2]. |
   | `Rocket.toml` | This tells Rocket what port to serve the API on[^3].     |

1. Configure `systemd` to run the executable as a service.
   * Copy `fortuneapi.service` to `/etc/systemd/system`
   * Enable the service: `$ sudo systemctl enable fortuneapi`
1. Configure Apache2 to serve the API as a reverse proxy.
   * Copy `fortuneapi.conf`[^4] to `/etc/apache2/sites-available`. 
   * Enable the reverse proxy: `sudo a2ensite fortuneapi.conf`[^5]

## Notes:
[^1] Since this app's data files are compatible with the BSD program (`fortune-mod`) and since that program puts them in `/usr/share/games/fortunes`, that might be a good location, as long as you don't mind each program using the same files.

[^2] This file *must* be edited with the correct path to the data files, presumably `/usr/share/games/fortune` as discussed in Note 1.

[^3] `Rocket.toml` is configured to use port 4040 for a release build.

[^4] Assumes the API will be using port 4040; Edit it if the port is configured differently in `Rocket.toml`. 

[^5] If all goes will, Apache will serve the API at http://marconi/api/fortune.
