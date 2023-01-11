package com.snake.game;

import java.awt.Graphics;
import java.util.ArrayList;

public interface Snake {
    public abstract void createSnake(Graphics g, int cellSize);
    //
    public abstract void move();
    //
    public abstract ArrayList<Position> Tiles();
    //
    public enum Direction {
        Left,
        Right,
        Up,
        Down
    }
}
