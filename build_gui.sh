#!/bin/bash

# Build script for Matrix Terminal GUI
set -e

# Directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Parse arguments
RELEASE=true
RUN=false
SIMPLE=false

for arg in "$@"; do
    case $arg in
        --debug)
            RELEASE=false
            ;;
        --run)
            RUN=true
            ;;
        --simple)
            SIMPLE=true
            ;;
        --help)
            echo "Usage: ./build_gui.sh [--debug] [--run] [--simple]"
            echo ""
            echo "Options:"
            echo "  --debug      Build in debug mode (default is release)"
            echo "  --run        Run the application after building"
            echo "  --simple     Build only the simple GUI prototype"
            echo "  --help       Show this help message"
            exit 0
            ;;
    esac
done

# Build configuration
if [ "$RELEASE" = true ]; then
    BUILD_OPT="--release"
    BUILD_DIR="release"
    echo -e "${GREEN}Building in release mode...${NC}"
else
    BUILD_OPT=""
    BUILD_DIR="debug"
    echo -e "${GREEN}Building in debug mode...${NC}"
fi

# Build the simple prototype
if [ "$SIMPLE" = true ] || [ "$RUN" = true ]; then
    echo -e "Building simple prototype..."
    cd "$SCRIPT_DIR/simple-gui"
    cargo build $BUILD_OPT
    
    # Copy the binary to a more accessible location
    mkdir -p "$SCRIPT_DIR/matrix-gui/target/$BUILD_DIR"
    cp "target/$BUILD_DIR/matrix_simple_gui" "$SCRIPT_DIR/matrix-gui/target/$BUILD_DIR/matrix_simple_gui"
fi

# Build the full application if not in simple mode
if [ "$SIMPLE" = false ]; then
    echo -e "Building full application..."
    cd "$SCRIPT_DIR/matrix-gui"
    cargo build $BUILD_OPT
fi

echo -e "${GREEN}Build completed successfully!${NC}"
echo -e "You can find the binaries at:"

if [ "$SIMPLE" = false ]; then
    echo -e "  - ${GREEN}matrix-gui/target/$BUILD_DIR/matrix_terminal_gui${NC} (full application)"
fi

if [ "$SIMPLE" = true ] || [ "$RUN" = true ]; then
    echo -e "  - ${GREEN}matrix-gui/target/$BUILD_DIR/matrix_simple_gui${NC} (simple prototype)"
    
    # Run the simple prototype if requested
    if [ "$RUN" = true ]; then
        echo -e "${GREEN}Running simple prototype...${NC}"
        "$SCRIPT_DIR/matrix-gui/target/$BUILD_DIR/matrix_simple_gui"
    fi
fi