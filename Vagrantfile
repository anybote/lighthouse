Vagrant.configure("2") do |config|
  # base box for the environment
  config.vm.box = "generic/ubuntu2204"

  config.vm.synced_folder ".", "/vagrant/lighthouse"

  # Create a forwarded port mapping which allows access to a specific port
  # within the machine from a port on the host machine and only allow access
  # via 127.0.0.1 to disable public access
  config.vm.network "forwarded_port", guest: 1234, host: 1234, host_ip: "127.0.0.1"
  config.vm.network "forwarded_port", guest: 8000, host: 8000, host_ip: "127.0.0.1"
  config.vm.network "forwarded_port", guest: 3000, host: 3000, host_ip: "0.0.0.0"

  # provisioning
  # sudo apt update && sudo apt install build-essential
  # curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  # copy pub ssh key
end
