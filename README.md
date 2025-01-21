# pact

## Info
Pact is a presidential action tracker.

It returns recent presidential actions, sourced from [the White House](https://www.whitehouse.gov/presidential-actions).

**Screenshots!!**
![Screenshot](https://github.com/user-attachments/assets/58f287bd-01c1-451f-8fb3-e145154aec7d)

## Install
This script does it all for you.

```bash
curl -fsSL 'https://github.com/Toxikuu/pact/raw/refs/heads/master/install.sh' -o /tmp/pact-install.sh && sudo bash /tmp/pact-install.sh && rm -vf /tmp/pact-install.sh
```

Once your curiosity has been sated, `sudo rm -vf /usr/bin/pact`.

## Usage
By default, pact shows the 10 most recent presidential actions:
```bash
pact
```

To view more pages (10 actions per page), pass -p NUM:
```bash
pact -p 4 # shows the 40 most recent presidential actions
```
