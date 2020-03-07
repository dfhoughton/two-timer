#! env ruby

# something to help examine the possible variations of a sub-pattern
# run without arguments to see usage
# mind the expontential complexity!

raw = <<-END.gsub %r{//.*$}, ''
        TOP -> r(r"\A") <time_expression> r(r"\z")

        // non-terminal patterns
        // these are roughly ordered by dependency

        time_expression => <universal> | <particular>

        particular => <one_time> | <two_times>

        one_time => <moment_or_period>

        two_times -> ("from")? <moment_or_period> <to> <moment_or_period>

        to => <up_to> | <through>

        moment_or_period => <moment> | <period>

        period => <named_period> | <specific_period>

        specific_period => <modified_period> | <month_and_year> | <year> | <relative_period>

        modified_period -> <modifier>? <modifiable_period>

        // replacing [["week", "month", "year", "pay period", "payperiod", "pp", "weekend"]] with <some_period>
        // because this script doesn't have handling for the [...] pattern
        modifiable_period => <some_period> | <a_month> | <a_day>

        month_and_year -> <a_month> <year>

        year => <short_year> | ("-")? <n_year>
        year -> <suffix_year> <year_suffix>

        year_suffix => <ce> | <bce>

        relative_period -> <count> <displacement> <from_now_or_ago>

        count => r(r"[1-9][0-9]*") | <a_count>

        named_period => <a_day> | <a_month>

        moment -> <adjustment>? <point_in_time>

        adjustment -> <amount> <direction> // two minutes before

        amount -> <count> <unit>

        point_in_time -> <at_time_on>? <some_day> <at_time>? | <specific_time> | <time>

        at_time_on -> ("at")? <time> ("on")?

        some_day => <specific_day> | <relative_day>

        specific_day => <adverb> | <date_with_year>

        date_with_year => <n_date> | <a_date>

        n_date -> <year>    r("[./-]") <n_month> r("[./-]") <n_day>
        n_date -> <year>    r("[./-]") <n_day>   r("[./-]") <n_month>
        n_date -> <n_month> r("[./-]") <n_day>   r("[./-]") <year>
        n_date -> <n_day>   r("[./-]") <n_month> r("[./-]") <year>

        a_date -> <day_prefix>? <a_month> <o_n_day> (",") <year>
        a_date -> <day_prefix>? <n_day> <a_month> <year>
        a_date -> <day_prefix>? ("the") <o_day> ("of") <a_month> <year>

        day_prefix => <a_day> (",")?

        relative_day => <a_day> | <a_day_in_month>

        at_time -> ("at") <time>

        specific_time => <first_time> | <last_time> | <precise_time>

        precise_time -> <n_date> <hour_24>

        time -> <hour_12> <am_pm>? | <hour_24> | <named_time>

        hour_12 => <h12>
        hour_12 => <h12> (":") <minute>
        hour_12 => <h12> (":") <minute> (":") <second>

        hour_24 => <h24>
        hour_24 => <h24> (":") <minute>
        hour_24 => <h24> (":") <minute> (":") <second>

        a_day_in_month => <ordinal_day> | <day_and_month>

        ordinal_day   -> <day_prefix>? ("the") <o_day>    // the first

        o_day => <n_ordinal> | <a_ordinal> | <roman>

        day_and_month -> <n_month> r("[./-]") <n_day>     // 5-6
        day_and_month -> <a_month> ("the")? <o_n_day>              // June 5, June 5th, June fifth, June the fifth
        day_and_month -> ("the") <o_day> ("of") <a_month> // the 5th of June, the fifth of June

        o_n_day => <n_day> | <o_day>
END

# convert this into some partially parsed raw data for further examination
RAW = {}.tap do |h|
  raw.lines.each do |l|
    l.strip!
    next unless l.length > 0
    key, value = l.split /\s*[=-]>\s*/
    list = h[key] ||= []
    value.split('|').each do |piece|
      list << piece.strip
    end
  end
end

#####
## monkey patching for convenience
#####

class String
  def deep_dup
    self
  end
  def show(depth=0)
    puts "  " * depth + self
  end
end

class Array
  def deep_dup
    map{ |o| o.deep_dup }
  end
  def show(depth=0)
    each do |o|
      o.show(depth)
    end
  end
end

class Hash
  def deep_dup
    Hash[
      to_a.map do |k,v|
        [ k.deep_dup, v.deep_dup ]
      end
    ]
  end
  def show(depth=0)
    each do |k,v|
      k.show(depth)
      v.show(depth + 1)
    end
  end
end

#####
## functions for extracting variants out of the grammar
#####

def augment(list, element)
  if list.length == 0
    [element.deep_dup]
  else
    list.map do |item|
      if Array === item
        item.deep_dup + [element.deep_dup]
      else
        [ item.deep_dup, element.deep_dup ]
      end
    end
  end
end

def variants(key)
  ( @variants ||= {} )[key] ||= begin
    all_vs = []
    ( RAW[key] || [] ).each do |line|
      vs = []
      line .scan(/(?:<\w+>|r?\([^)]+\))\??/).each do |element|
        optional = element[-1] == '?'
        if optional
          element = element[0..-2]
          without = vs.dup
        end
        case element[0]
        when 'r', '('
          vs = augment(vs, element)
        when '<'
          subkey = element[1..-2]
          if RAW.include? subkey
            new_vs = []
            variants(subkey).each do |v|
              new_vs += augment(vs, { subkey => v })
            end
            vs = new_vs
          else
            vs = augment(vs, subkey)
          end
        end
        if optional
          vs = without + vs
        end
      end
      all_vs += vs
    end
    all_vs
  end
end

# convert a variant tree into its leaves
def leaves(obj)
  case obj
  when Array
    obj.map{ |o| leaves o }.flatten
  when Hash
    obj.values.map{ |o| leaves o }.flatten
  else
    [obj]
  end
end

#####
## the main function
#####

key = ARGV.shift

if key
  variants(key).each do |v|
    puts leaves(v).join(' ')
    puts "-----"
    v.show
    puts
  end
else
  puts "USAGE: provide one of the following keys to see all the variants in the grammar for that key\n\n"
  RAW.keys.sort.each{ |k| puts k }
end