/*
 * niepce - modules/darkroom/darkroommodule.h
 *
 * Copyright (C) 2008 Hubert Figuiere
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




#ifndef _DARKROOM_MODULE_H__
#define _DARKROOM_MODULE_H__

#include <gtkmm/widget.h>
#include <gtkmm/paned.h>
#include <gtkmm/box.h>
#include <gtkmm/actiongroup.h>
#include <gtkmm/scrolledwindow.h>

#include "fwk/toolkit/controller.h"
#include "db/libfile.h"
#include "libraryclient/libraryclient.h"
#include "ncr/image.h"
#include "modules/darkroom/imagecanvas.h"
#include "modules/darkroom/toolboxcontroller.h"
	
namespace framework {
class Dock;
}

namespace darkroom {

class DarkroomModule
	: public framework::Controller
{
public:
	typedef boost::shared_ptr<DarkroomModule> Ptr;

	DarkroomModule(const Glib::RefPtr<Gtk::ActionGroup> & action_group,
                   const libraryclient::LibraryClient::Ptr & client)
        : m_actionGroup(action_group),
          m_image(new ncr::Image),
          m_libClient(client)
		{
		}

	void set_image(const db::LibFile::Ptr & file);

protected:
	virtual Gtk::Widget * buildWidget();

private:
    // darkroom split view
    Gtk::HPaned                  m_dr_splitview;
    Gtk::VBox                    m_vbox;
    ImageCanvas*                 m_imagecanvas;
    Gtk::ScrolledWindow          m_canvas_scroll;
    ToolboxController::Ptr       m_toolbox_ctrl;
    Glib::RefPtr<Gtk::ActionGroup> m_actionGroup;
    ncr::Image::Ptr              m_image;
    libraryclient::LibraryClient::Ptr m_libClient;
    framework::Dock              *m_dock;
};


}

#endif
/*
  Local Variables:
  mode:c++
  c-file-style:"stroustrup"
  c-file-offsets:((innamespace . 0))
  indent-tabs-mode:nil
  fill-column:80
  End:
*/