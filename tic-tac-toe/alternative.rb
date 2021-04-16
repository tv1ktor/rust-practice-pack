# I was wondering how much code and time it would take to repeat the same task by using highlevel language.
# Actually much faster. Howerver, I have to admit, even though I am just lerning Rust, it was fairly easy
# to get it done, especially when `ownership` concept does not raise any questions.

class Board
  def initialize
    @rows = [
      ['1', '2', '3'],
      ['4', '5', '6'],
      ['7', '8', '9']
    ]
  end

  def occupy(row, col, mark)
    @rows[row][col] = mark
  end

  def print
    separator = '+---+---+---+'

    @rows.each do |row|
      puts(separator, "| #{row[0]} | #{row[1]} | #{row[2]} |")
    end

    puts separator
  end

  def values_at(*positions)
    vals = []

    positions.each { |p|
      row, col = get_row_and_column(p)
      vals << @rows[row][col]
    }

    vals
  end

  def get_row_and_column(pos)
    [
      (pos - 1) / 3,
      (pos - 1) % 3
    ]
  end

  def validate_input(pos)
    row, col = get_row_and_column(pos)

    case @rows[row]&.[](col)
    when 'X', 'O'
      [false, nil]
    when nil
      raise ArgumentError, 'Position out of range'
    else
      [true, [row, col]]
    end
  end
end

class Turn
  Player = Class.new
  Bot = Class.new

  def initialize
    @owner = Player
  end

  attr_reader :owner

  def switch
    @owner = @owner.equal?(Bot) ? Player : Bot
  end

  def marker
    @owner.equal?(Bot) ? 'O' : 'X'
  end
end

class Game
  def initialize
    @board = Board.new
    @current_turn = Turn.new
  end

  def start
    finished = false

    until finished
      turn

      if game_over?
        puts "#{current_turn.owner} is a winner"
        finished = true
      else
        current_turn.switch
      end
    end
  end

  attr_reader :board, :current_turn

  def turn
    board.print
    print "\nPlease enter the position number: "
    position = gets.then(&:to_i)

    raise ArgumentError, 'Not valid number' unless position

    success, pos_data = board.validate_input(position)

    raise ArgumentError, 'Already taken' unless success

    board.occupy(pos_data[0], pos_data[1], current_turn.marker)
  end


  def game_over?
    winning_options = [
      [1, 2, 3],
      [4, 5, 6],
      [7, 8, 9],
      [1, 4, 7],
      [2, 5, 8],
      [3, 6, 9],
      [1, 5, 9],
      [7, 5, 9]
    ]

    winning_options.each do |positions|
      return true if board.values_at(*positions).uniq.size == 1
    end

    false
  end
end

g = Game.new
g.start