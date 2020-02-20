## VPN box with no VPN Server running

# Network diagram

![VPN](vpn-server.png)


# Setup
"A" is my VPN box, with no VPN server running... This box is accessible from outside via port forwarding. With VPN, (right or wrong), this box serves as ssh jumping host. This host is accesible from inside the network, via ssh and lxc console.

# question
Should there be any traffic?


# net-gazer setup
1. net-gazer-sensor sniffs traffic from 'eth0' nic and sends reports via 'lo'
2. traceroute plugin deployed to net-gazer
3. net-gazer-web runs on that box, on 'lo' nic. There is not db running, but transaction log, that I will replay later into database


I will leave it running for 24 hours - let's see what happens.....
