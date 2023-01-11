package com.snake.game;

import java.awt.Graphics;
import java.util.ArrayList;
import java.awt.Color;

public class Snek implements Snake {
    public Position[] position = new Position[] {
        new Position(2, 10),
        new Position(3, 10),
        new Position(4, 10)
    };
    //
    @Override
    public void createSnake(Graphics g, int cellSize) {
        //
        g.setColor(Color.GREEN);
        //
        for (Position p : Tiles()) {
            g.fillRect(p.x*cellSize, p.y*cellSize, cellSize, cellSize);
        }
    }
    Direction currentDirection = Direction.Right;
    //
    @Override
    public void move() {
        // moves snake
        for (int i = 0; i < position.length - 1; i++) {
            position[i] = position[i + 1];
        }
        switch(currentDirection) {
            case Right:
                position[position.length - 1] = new Position(position[position.length - 1].x + 1, position[position.length - 1].y);
                break;
            case Left:
                position[position.length - 1] = new Position(position[position.length - 1].x - 1, position[position.length - 1].y);
                break;
            case Down:
                position[position.length - 1] = new Position(position[position.length - 1].x, position[position.length - 1].y + 1);
                break;
            case Up:
                position[position.length - 1] = new Position(position[position.length - 1].x, position[position.length - 1].y - 1);
                break;
            default:
                break;
        }
    }
    @Override
    public ArrayList<Position> Tiles() {
        ArrayList<Position> positions = new ArrayList<>();
        for (Position pos : position) {
            positions.add(new Position(pos.x, pos.y));
        }
        return positions;
    }
}
