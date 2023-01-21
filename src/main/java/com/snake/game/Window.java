package com.snake.game;

import javax.swing.JFrame;
import javax.swing.JPanel;
import javax.swing.Timer;
import com.snake.game.Snake.Direction;
import java.awt.Graphics;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;
import java.util.Random;
import java.awt.Color;

public class Window extends JFrame {
    private Snek snek = new Snek();
    private SnakePanel sPanel;
    private Apple apple;
    private Panel panel;
    //
    public Window(int width, int height) {
        frameConfig(width, height);
        this.setFocusable(true);
        //
        this.add(sPanel);
        this.add(panel);
        //
        Timer timer = new Timer(400, e -> {
            if(isAppleEaten()) {
                snek.addSnakePart();
                apple.setRandomPos(randomApplePos());
                return;
            }
            snek.move();
            repaint();
        });
        timer.setRepeats(true);
        timer.start();
    }
    private boolean isAppleEaten() {
        if(snek.Tiles().get(snek.Tiles().size() - 1).x == apple.getX() && snek.Tiles().get(snek.Tiles().size() - 1).y == apple.getY()) {
            return true;
        } else {
            return false;
        }
    }
    public int randomApplePos() {
        int randomPosition = new Random().nextInt(0, 18);
        for (Position p : snek.Tiles()) {
            if(p.x == randomPosition && p.y == randomPosition) {
                randomPosition = new Random().nextInt(0, 18);
            }
        }
        return randomPosition;
    }
    //
    public void frameConfig(int width, int height) {
        apple = new Apple(randomApplePos());
        panel = new Panel();
        sPanel = new SnakePanel();
        //
        this.setSize(width, height);
        this.setDefaultCloseOperation(EXIT_ON_CLOSE);
        this.setLocationRelativeTo(null);
        this.setTitle("Snek... or something");
        this.setBackground(Color.BLACK);
        this.addKeyListener(new kl());
        this.setVisible(true);
        //
    }
    class kl implements KeyListener {
        @Override
        public void keyTyped(KeyEvent e) {}
        @Override
        public void keyPressed(KeyEvent e) {
            int keycode = e.getKeyCode();
            //
            if(keycode == KeyEvent.VK_UP && snek.currentDirection != Direction.Down) {
                snek.currentDirection = Direction.Up;
            } else if(keycode == KeyEvent.VK_DOWN && snek.currentDirection != Direction.Up) {
                snek.currentDirection = Direction.Down;
            } else if(keycode == KeyEvent.VK_LEFT && snek.currentDirection != Direction.Right) {
                snek.currentDirection = Direction.Left;
            } else if(keycode == KeyEvent.VK_RIGHT && snek.currentDirection != Direction.Left) {
                snek.currentDirection = Direction.Right;
            }
        }
        @Override
        public void keyReleased(KeyEvent e) {}
    }
    class SnakePanel extends JPanel {
        //
        public SnakePanel() {
            this.setSize(600,600);
            this.setDoubleBuffered(true);
            this.setFocusable(false);
            this.setOpaque(false);
        }
        //
        public void paint(Graphics g) {
            super.paint(g);
            //
            apple.draw(g);
            snek.createSnake(g);
        }
    }
}
