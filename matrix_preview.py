#!/usr/bin/env python3
import curses
import time

def draw_sidebar(stdscr, height):
    """Draw the Matrix-style sidebar"""
    # Define Matrix green color pair
    curses.init_pair(1, curses.COLOR_GREEN, curses.COLOR_BLACK)
    matrix_green = curses.color_pair(1)
    
    # Draw sidebar background
    for y in range(height):
        stdscr.addstr(y, 0, "│", matrix_green)
    
    # Draw sidebar icons
    icons = ["N", "H", "V", "G", "=", "‖", "M", "Z", "X", "?"]
    for i, icon in enumerate(icons):
        y_pos = i * 3 + 1
        if y_pos < height:
            stdscr.addstr(y_pos, 0, icon, matrix_green | curses.A_BOLD)

def draw_tooltip(stdscr, name, description, shortcut):
    """Draw a Matrix-style tooltip"""
    curses.init_pair(1, curses.COLOR_GREEN, curses.COLOR_BLACK)
    matrix_green = curses.color_pair(1)
    
    # Draw tooltip box
    x, y = 4, 3
    width, height = 40, 7
    
    # Draw the box
    stdscr.addstr(y, x, "┌" + "─" * (width - 2) + "┐", matrix_green)
    for i in range(1, height - 1):
        stdscr.addstr(y + i, x, "│", matrix_green)
        stdscr.addstr(y + i, x + width - 1, "│", matrix_green)
    stdscr.addstr(y + height - 1, x, "└" + "─" * (width - 2) + "┘", matrix_green)
    
    # Add content
    stdscr.addstr(y + 1, x + 2, name, matrix_green | curses.A_BOLD)
    stdscr.addstr(y + 3, x + 2, description, matrix_green)
    stdscr.addstr(y + 5, x + 2, f"Shortcut: {shortcut}", matrix_green)

def draw_terminal_window(stdscr, x, y, width, height, title, is_focused=False):
    """Draw a terminal window"""
    curses.init_pair(1, curses.COLOR_GREEN, curses.COLOR_BLACK)
    curses.init_pair(2, curses.COLOR_CYAN, curses.COLOR_BLACK)
    
    matrix_green = curses.color_pair(1)
    focused_color = curses.color_pair(2) if is_focused else curses.color_pair(1)
    
    # Draw the border
    stdscr.addstr(y, x, "┌" + "─" * (width - 2) + "┐", focused_color)
    for i in range(1, height - 1):
        stdscr.addstr(y + i, x, "│", focused_color)
        stdscr.addstr(y + i, x + width - 1, "│", focused_color)
    stdscr.addstr(y + height - 1, x, "└" + "─" * (width - 2) + "┘", focused_color)
    
    # Draw the title
    if len(title) > width - 4:
        title = title[:width - 7] + "..."
    stdscr.addstr(y, x + 2, title, focused_color)
    
    # Add some content
    if is_focused:
        stdscr.addstr(y + 2, x + 2, "Welcome to Matrix Terminal!", matrix_green)
        stdscr.addstr(y + 3, x + 2, "The Matrix is all around us...", matrix_green)
        stdscr.addstr(y + 5, x + 2, "> _", matrix_green)
    else:
        stdscr.addstr(y + 2, x + 2, "Matrix process running...", matrix_green)
        stdscr.addstr(y + 5, x + 2, "> exit", matrix_green)

def main(stdscr):
    # Clear the screen
    stdscr.clear()
    curses.curs_set(0)  # Hide cursor
    curses.start_color()
    
    # Get screen dimensions
    height, width = stdscr.getmaxyx()
    
    # Draw sidebar
    draw_sidebar(stdscr, height)
    
    # Draw tooltip for a hovered icon
    draw_tooltip(stdscr, "Split Horizontal", "Split current window horizontally", "Ctrl+H or :split h")
    
    # Draw terminal windows
    sidebar_width = 2
    main_width = width - sidebar_width
    
    # Draw a focused main window
    draw_terminal_window(stdscr, sidebar_width, 0, main_width // 2, height // 2, "Matrix Terminal [Running]", True)
    
    # Draw additional windows
    draw_terminal_window(stdscr, sidebar_width + main_width // 2, 0, main_width // 2, height // 2, "Secondary Terminal [Running]")
    draw_terminal_window(stdscr, sidebar_width, height // 2, main_width, height // 2, "Terminal 3 [Running]")
    
    # Update the screen
    stdscr.refresh()
    time.sleep(5)

if __name__ == "__main__":
    curses.wrapper(main)