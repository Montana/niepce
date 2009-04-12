/*
 * niepce - fwk/toolkit/frame.hpp
 *
 * Copyright (C) 2007-2008 Hubert Figuiere
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */


#ifndef _FRAMEWORK_FRAME_H_
#define _FRAMEWORK_FRAME_H_

#include <string>
#include <boost/function.hpp>

#include <sigc++/signal.h>
#include <gtkmm/toggleaction.h>
#include <gtkmm/builder.h>

#include "fwk/toolkit/controller.hpp"

namespace Gtk {
	class Dialog;
}

namespace fwk {

class Frame 
		: public Controller
{
public:
		typedef boost::shared_ptr<Frame> Ptr;

		Frame(const std::string & gladeFile, const Glib::ustring & widgetName,
          const std::string & layout_cfg_key = "");
		Frame(const std::string & layout_cfg_key = "");
		virtual ~Frame();

    /** convenience to return the Frame::Ptr from this */
    Ptr shared_frame_ptr()
        {
            return boost::static_pointer_cast<Frame>(shared_from_this());
        }

		Gtk::Window & gtkWindow()
        {
            return *m_window; 
        }
    Glib::RefPtr<Gtk::Builder> & builder()
        { return m_builder; }

		/** set the title of the window.
		 * @param title the title of the window.
		 * 
		 * override to provide you own hooks - behaviour.
		 */
		virtual void set_title(const std::string & title);
		/** set the window icon from the theme 
		 * @param name the icon name in the theme
		 */
		void set_icon_from_theme(const Glib::ustring & name);

    void toggle_tools_visible();

    sigc::signal<void> signal_hide_tools;
    sigc::signal<void> signal_show_tools;
protected:
		/** close signal handler */
		virtual bool _close();
    Glib::RefPtr<Gtk::ToggleAction> m_hide_tools_action;

private:
    /** frame have the widget set at construction time
     * from a ui file or directly.
     */
    virtual Gtk::Widget *buildWidget();
		void connectSignals();
		void frameRectFromConfig();
		void frameRectToConfig();

		Gtk::Window *m_window;
    Glib::RefPtr<Gtk::Builder> m_builder;
		std::string m_layout_cfg_key;
};

}


/*
  Local Variables:
  mode:c++
  c-file-style:"stroustrup"
  c-file-offsets:((innamespace . 0))
  indent-tabs-mode:nil
  fill-column:80
  End:
*/
#endif
