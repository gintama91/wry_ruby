require "test_helper"

class TestWryRuby < Minitest::Test
  def test_tray_id_new_with_empty_string
    assert_raises(StandardError) do
      TrayId.new("") # Unique string is empty
    end
  end

  def test_tray_id_new_with_non_empty_string
    tray_id = TrayId.new("hmm") # Unique string is non-empty
    refute(tray_id.is_empty)
  end

  def test_tray_id_new_with_non_interned_string
    error = assert_raises(StandardError) do
      TrayId.new("non_interned_string") # Unique string is not interned
    end
    assert_equal "Interned string not available", error.message
  end
end
