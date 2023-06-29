require 'test_helper'

class ClipboardTest < Minitest::Test
  def test_clipboard_functionality
    clip = Clip.new

    # Write text to the clipboard
    clip.write_text("Hello, World!")

    # Read text from the clipboard
    text = clip.read_text

    assert_equal("Hello, World!", text)
  end
end
