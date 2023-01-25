package com.snake.game;

import java.awt.Graphics;
import java.util.ArrayList;
import java.awt.Color;

public class Snek implements Snake {
    private final int cellSize = 32;
    public Position[] position = new Position[] {
        new Position(4, 10),
        new Position(3, 10),
        new Position(2, 10)
    };
    //
    public void createSnake(Graphics g) {
        //
        g.setColor(Color.GREEN);
        for (int i = position.length; i > 0; i--) {    
            if(i == 1) g.fillRect(position[0].x*cellSize, position[0].y*cellSize, cellSize, cellSize);
            g.drawRect(position[i - 1].x*cellSize, position[i - 1].y*cellSize, cellSize, cellSize);
        }
    }
    Direction currentDirection = Direction.Right;
    //
    @Override
    public void move() {
        // moves snake
        for (int i = position.length - 1; i > 0; i--) {
            position[i] = position[i - 1];
        }
        switch(currentDirection) {
            case Right:
                position[0] = new Position(position[0].x + 1, position[0].y);
                break;
            case Left:
                position[0] = new Position(position[0].x - 1, position[0].y);
                break;
            case Down:
                position[0] = new Position(position[0].x, position[0].y + 1);
                break;
            case Up:
                position[0] = new Position(position[0].x, position[0].y - 1);
                break;
            default:
                break;
        }
    }
    public void addSnakePart() {
        Position[] clone = new Position[position.length + 1];
        for (int i = 0; i < position.length; i++) {
            clone[i + 1] = position[i];
        }
        switch(currentDirection) {
            case Up:
                clone[0] = new Position(clone[1].x, clone[1].y - 1);
                break;
            case Down:
                clone[0] = new Position(clone[1].x, clone[1].y + 1);
                break;
            case Left:
                clone[0] = new Position(clone[1].x - 1, clone[1].y);
                break;
            case Right:
                clone[0] = new Position(clone[1].x + 1, clone[1].y);
                break;
            default:
                break;
        }
        position = clone;
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
