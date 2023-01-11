package com.snake.game;

import javax.swing.JFrame;
import javax.swing.JPanel;
import javax.swing.Timer;

import com.snake.game.Snake.Direction;

import java.awt.Graphics;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;
import java.awt.Color;

public class Window extends JFrame implements KeyListener {
    private Snek snek = new Snek();
    private SnakePanel sPanel;
    private Panel panel;
    //
    public Window(int width, int height) {
        frameConfig(width, height);
        this.setFocusable(true);
        //
        this.add(sPanel);
        this.add(panel);
        //
        Timer timer = new Timer(600, e -> {
            snek.move();
            repaint();
        });
        timer.setRepeats(true);
        timer.start();
    }
    //
    public void frameConfig(int width, int height) {
        panel = new Panel(snek);
        sPanel = new SnakePanel();
        //
        this.setSize(width, height);
        this.setDefaultCloseOperation(EXIT_ON_CLOSE);
        this.setLocationRelativeTo(null);
        this.setTitle("Snek... or something");
        this.setBackground(Color.BLACK);
        this.addKeyListener(this);
        this.setVisible(true);
        //
    }
    class SnakePanel extends JPanel {
        //
        public SnakePanel() {
            this.setSize(600,600);
            this.setFocusable(false);
            this.setOpaque(false);
        }
        //
        public void paint(Graphics g) {
            super.paint(g);
            //
            snek.createSnake(g, 32);
        }
    }
    @Override
    public void keyTyped(KeyEvent e) {}
    @Override
    public void keyPressed(KeyEvent e) {
        int keycode = e.getKeyCode();
        if(keycode == KeyEvent.VK_UP && snek.currentDirection != Direction.Down) {
            snek.currentDirection = Direction.Up;
        } else if(keycode == KeyEvent.VK_DOWN && snek.currentDirection != Direction.Up) {
            snek.currentDirection = Direction.Down;
        } else if(keycode == KeyEvent.VK_LEFT && snek.currentDirection != Direction.Right) {
            snek.currentDirection = Direction.Left;
        } else if(keycode == KeyEvent.VK_RIGHT && snek.currentDirection != Direction.Left) {
            snek.currentDirection = Direction.Right;
        }
        repaint();
    }
    @Override
    public void keyReleased(KeyEvent e) {}
}
