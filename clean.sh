#!/usr/bin/env bash

function clean_cargo_projects {
    for dir in */; do
        if [ -d "$dir" ]; then
            cd "$dir" || continue
            if [ -f "Cargo.toml" ]; then
                echo "Running 'cargo clean' in $(pwd)"
                cargo clean
            else
                echo "No Cargo.toml found in $(pwd), skipping..."
            fi
            cd .. || exit
        fi
    done
}

clean_cargo_projects
