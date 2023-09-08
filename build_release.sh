echo "Updating rust targets"
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
echo "Rust targets updated"
echo "=========================="
echo "Building for Windows"
cargo build --release --target x86_64-pc-windows-gnu
echo "Done!"
echo "=========================="#!/bin/bash

if [ ! -f build.conf ]; then
  echo "File build.conf does not exist, creating it."
  echo "UpdateRustTargets=true" >>build.conf
  echo "BuildWindows=true" >>build.conf
  echo "BuildLinux=true" >>build.conf
  echo "BuildMac=true" >>build.conf
  exit 0
fi

UpdateRustTargets=$(cat build.conf | grep UpdateRustTargets | cut -d "=" -f 2)
BuildWindows=$(cat build.conf | grep BuildWindows | cut -d "=" -f 2)
BuildLinux=$(cat build.conf | grep BuildLinux | cut -d "=" -f 2)
BuildMac=$(cat build.conf | grep BuildMac | cut -d "=" -f 2)

if [ "$UpdateRustTargets" == "true" ]; then
  echo -e "\e[94mUpdating rust targets\e[0m"
  rustup target add x86_64-pc-windows-gnu
  rustup target add x86_64-unknown-linux-gnu
  rustup target add x86_64-apple-darwin
  if [ $? -ne 0 ]; then
    echo -e "\e[31mError: Failed to update rust targets\e[0m"
  else
    echo -e "\e[92mRust targets updated\e[0m"
  fi
elif [ "$UpdateRustTargets" == "false" ]; then
  echo -e "\e[94mSkipping rust target update.\e[0m"
else
  echo -e "\e[31mError: Invalid value for UpdateRustTargets: $UpdateRustTargets\e[0m"
fi

if [ "$BuildWindows" == "true" ]; then
  echo -e "\e[94mBuilding for Windows\e[0m"
  cargo build --release --target x86_64-pc-windows-gnu
  if [ $? -ne 0 ]; then
    echo -e "\e[31mError: Failed to build for Windows\e[0m"
  else
    echo -e "\e[92mDone!\e[0m"
  fi
elif [ "$BuildWindows" == "false" ]; then
  echo -e "\e[94mSkipping Windows build.\e[0m"
else
  echo -e "\e[31mError: Invalid value for BuildWindows: $BuildWindows\e[0m"
fi

if [ "$BuildLinux" == "true" ]; then
  echo -e "\e[94mBuilding for Linux\e[0m"
  cargo build --release --target x86_64-unknown-linux-gnu
  if [ $? -ne 0 ]; then
    echo -e "\e[31mError: Failed to build for Linux\e[0m"
  else
    echo -e "\e[92mDone!\e[0m"
  fi
elif [ "$BuildLinux" == "false" ]; then
  echo -e "\e[94mSkipping Linux build.\e[0m"
else
  echo -e "\e[31mError: Invalid value for BuildLinux: $BuildLinux\e[0m"
fi

if [ "$BuildMac" == "true" ]; then
  echo -e "\e[94mBuilding for Mac\e[0m"
  cargo build --target x86_64-apple-darwin
  if [ $? -ne 0 ]; then
    echo -e "\e[31mError: Failed to build for Mac\e[0m"
  else
    echo -e "\e[92mDone!\e[0m"
  fi
elif [ "$BuildMac" == "false" ]; then
  echo -e "\e[94mSkipping Mac build.\e[0m"
else
  echo -e "\e[31mError: Invalid value for BuildMac: $BuildMac\e[0m"
fi
echo "Building for Linux"
cargo build --release --target x86_64-unknown-linux-gnu
echo "Done!"
