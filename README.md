# Hug-Reminder-app
A reminder system to help remind me to hug others.

## Initialize database:
```
cargo run --bin db-init
```

## Build app:
```
cargo --release build --bin app
```

## SystemD Stuff:
There are many directories where you can place your unit and timer files. I put mine in the `/run/systemd/system` directory.

### Unit file
The service file defined how to run the application.

I have created a /hug_app directory at the root of my file directory and placed a binary of the app and a sqlite database for the app to use.

### Timer file
The timer file designates when to run the application.

I use the following:
```
Mon..Sun *-*-* 10..22:00,30:00
```
This should be Monday-Sunday everyday from 10am to 10pm every 30 minutes (00 minutes and 30 minutes).

### Helpful commands:

#### Run the service (out of band):
```
systemctl start hug
```

#### Status:
```
❯ systemctl status hug
● hug.service - Hug Reminder app
     Loaded: loaded (/run/systemd/system/hug.service; enabled; vendor preset: enabled)
     Active: inactive (dead) since Mon 2022-06-20 20:00:01 EDT; 1min 28s ago
TriggeredBy: ● hug.timer
       Docs: https://github.com/crazcalm/Hug-Reminder-app
    Process: 717769 ExecStart=/bin/sh -c cd /home/crazcalm/hug_app && ./app -e prod.env (code=exited, status=0/SUCCESS)
   Main PID: 717769 (code=exited, status=0/SUCCESS)

Jun 20 20:00:01 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC systemd[1]: Started Hug Reminder app.
Jun 20 20:00:01 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC sh[717770]: We are not sending an email
Jun 20 20:00:01 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC systemd[1]: hug.service: Succeeded.

```

```
❯ systemctl status hug.timer
● hug.timer - Runs the hug reminder app
     Loaded: loaded (/run/systemd/system/hug.timer; enabled; vendor preset: enabled)
     Active: active (waiting) since Mon 2022-06-20 19:50:08 EDT; 12min ago
    Trigger: Mon 2022-06-20 20:30:00 EDT; 27min left
   Triggers: ● hug.service

Jun 20 19:50:08 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC systemd[1]: Started Runs the hug reminder app.
```

#### Logs:
```
❯ journalctl -u hug
-- Logs begin at Tue 2022-03-29 14:38:47 EDT, end at Mon 2022-06-20 20:05:35 EDT. --
Jun 20 19:49:27 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC systemd[1]: Started Hug Reminder app.
Jun 20 19:49:31 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC sh[717161]: Email was sent
Jun 20 19:49:31 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC systemd[1]: hug.service: Succeeded.
Jun 20 20:00:01 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC systemd[1]: Started Hug Reminder app.
Jun 20 20:00:01 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC sh[717770]: We are not sending an email
Jun 20 20:00:01 crazcalm-HP-Compaq-Elite-8300-All-in-One-PC systemd[1]: hug.service: Succeeded.
```